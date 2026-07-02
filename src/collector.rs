//! Rust port of `python_example/collect.py`.
//!
//! Flattens a LaTeX article with `latexpand` and collects everything it
//! references (figures, local `.sty` packages, bibliography `.bib`/`.bbl`)
//! into a self-contained submission bundle, rewriting relative paths that
//! would otherwise climb out of the output directory.

use std::fs;
use std::path::Path;
use std::process::Command;
use std::sync::LazyLock;

use anyhow::{bail, Context, Result};
use regex::{Captures, Regex};

static INCLUDEGRAPHICS_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"\\includegraphics(\[[^\]]*\])?\{([^}]*)\}")
        .expect("INCLUDEGRAPHICS_RE pattern is a valid, hardcoded regex")
});

static USEPACKAGE_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"\\usepackage(?:\[[^\]]*\])?\{([^}]*)\}")
        .expect("USEPACKAGE_RE pattern is a valid, hardcoded regex")
});

static BIBLIOGRAPHY_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"\\bibliography\{([^}]*)\}")
        .expect("BIBLIOGRAPHY_RE pattern is a valid, hardcoded regex")
});

static LEADING_PARENT_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^(?:\.\./)+").expect("LEADING_PARENT_RE pattern is a valid, hardcoded regex")
});

/// Run `latexpand` against `input_path`, from within its parent directory.
///
/// latexpand resolves `\input`/`\include` relative to its CWD, not the
/// location of the file being processed, so it must be run from there.
///
/// `input_path` must already be an absolute/canonical path with a
/// non-empty parent directory (see `main::resolve_paths`).
pub fn call_latexpand(input_path: &Path) -> Result<String> {
    let file_name = input_path
        .file_name()
        .with_context(|| format!("{} has no file name", input_path.display()))?;

    let input_dir = input_path
        .parent()
        .with_context(|| format!("{} has no parent directory", input_path.display()))?;

    let output = Command::new("latexpand")
        .arg(file_name)
        .current_dir(input_dir)
        .output()
        .context("failed to spawn `latexpand` - is it installed and on PATH?")?;

    if !output.status.success() {
        bail!(
            "latexpand exited with {}: {}",
            output.status,
            String::from_utf8_lossy(&output.stderr)
        );
    }

    Ok(String::from_utf8(output.stdout)?)
}

pub fn save_to_file(output: &str, path: &Path) -> Result<()> {
    fs::write(path, output).with_context(|| format!("failed to write {}", path.display()))
}

/// Copy a single `\`-escaped relative path referenced from `input_dir` into
/// `output_dir`, stripping any leading `../` so the result no longer climbs
/// out of `output_dir` (e.g. `../figures_tables/x.pdf` -> `figures_tables/x.pdf`).
///
/// Returns the rewritten relative path text on success, or `None` if the
/// source file doesn't exist (a warning is logged in that case).
fn relocate_relative_file(raw_path: &str, input_dir: &Path, output_dir: &Path) -> Option<String> {
    let remainder = LEADING_PARENT_RE.replace(raw_path, "").into_owned();

    // LaTeX filenames sometimes escape underscores (e.g. "shap\_bar.pdf")
    // even though the file on disk has a literal underscore.
    let source = input_dir.join(raw_path.replace(r"\_", "_"));
    let delatexed_remainder = remainder.replace(r"\_", "_");

    if !source.is_file() {
        log::warn!("[collect] file not found, leaving path as-is: {raw_path}");
        return None;
    }

    let dest = output_dir.join(&delatexed_remainder);
    if let Some(parent) = dest.parent() {
        if let Err(err) = fs::create_dir_all(parent) {
            log::warn!("[collect] failed to create {}: {err}", parent.display());
            return None;
        }
    }
    if let Err(err) = fs::copy(&source, &dest) {
        log::warn!(
            "[collect] failed to copy {} -> {}: {err}",
            source.display(),
            dest.display()
        );
        return None;
    }

    Some(remainder)
}

/// Copy every `\includegraphics` target referenced in `tex_source` from
/// `input_dir` into `output_dir`, rewriting paths so they no longer climb
/// out of `output_dir`.
pub fn collect_media_files(tex_source: &str, input_dir: &Path, output_dir: &Path) -> String {
    let mut copied = 0u32;
    let mut warned = 0u32;

    let rewritten = INCLUDEGRAPHICS_RE.replace_all(tex_source, |caps: &Captures| {
        let options = caps.get(1).map_or("", |m| m.as_str());
        let raw_path = &caps[2];

        match relocate_relative_file(raw_path, input_dir, output_dir) {
            Some(relocated) => {
                copied += 1;
                format!(r"\includegraphics{options}{{{relocated}}}")
            }
            None => {
                warned += 1;
                caps[0].to_string()
            }
        }
    });

    log::info!("[collect] Copied {copied} figure(s), {warned} warning(s).");
    rewritten.into_owned()
}

/// Copy any local `.sty` file referenced via `\usepackage{...}` that lives
/// alongside the main .tex file into `output_dir`. Package names don't
/// encode a path, so no text rewriting is needed.
pub fn copy_local_packages(tex_source: &str, input_dir: &Path, output_dir: &Path) -> String {
    let mut copied: Vec<String> = Vec::new();

    for caps in USEPACKAGE_RE.captures_iter(tex_source) {
        for name in caps[1].split(',') {
            let name = name.trim();
            let source = input_dir.join(format!("{name}.sty"));
            if !source.is_file() {
                continue;
            }
            let Some(file_name) = source.file_name() else {
                continue;
            };
            if fs::copy(&source, output_dir.join(file_name)).is_ok() {
                copied.push(file_name.to_string_lossy().into_owned());
            }
        }
    }

    let joined = if copied.is_empty() {
        "-".to_string()
    } else {
        copied.join(", ")
    };
    log::info!(
        "[collect] Copied {} local package(s): {joined}.",
        copied.len()
    );

    tex_source.to_string()
}

/// Copy any locally-resolvable `\bibliography{...}` entries (e.g.
/// `../sources.bib`) into the bundle and rewrite their path, leaving
/// distribution-provided entries (e.g. `IEEEabrv`) untouched. Also copies
/// the already-built `<input stem>.bbl`, renamed to match the output stem,
/// so the bibliography compiles without re-running bibtex.
pub fn copy_bibliography(tex_source: &str, input_path: &Path, output_path: &Path) -> String {
    let input_dir = input_path.parent().unwrap_or_else(|| Path::new("."));
    let output_dir = output_path.parent().unwrap_or_else(|| Path::new("."));

    let rewritten = BIBLIOGRAPHY_RE.replace_all(tex_source, |caps: &Captures| {
        let rewritten_entries: Vec<String> = caps[1]
            .split(',')
            .map(str::trim)
            .map(|entry| {
                let has_extension = entry.ends_with(".bib");
                let raw_path = if has_extension {
                    entry.to_string()
                } else {
                    format!("{entry}.bib")
                };

                match relocate_relative_file(&raw_path, input_dir, output_dir) {
                    Some(relocated) if has_extension => relocated,
                    Some(relocated) => relocated
                        .strip_suffix(".bib")
                        .unwrap_or(&relocated)
                        .to_string(),
                    None => entry.to_string(),
                }
            })
            .collect();

        format!(r"\bibliography{{{}}}", rewritten_entries.join(","))
    });

    let bbl_source = input_path.with_extension("bbl");
    let bbl_dest = output_path.with_extension("bbl");
    if bbl_source.is_file() {
        match fs::copy(&bbl_source, &bbl_dest) {
            Ok(_) => log::info!(
                "[collect] Copied bibliography: {} -> {}.",
                bbl_source.file_name().unwrap_or_default().to_string_lossy(),
                bbl_dest.file_name().unwrap_or_default().to_string_lossy()
            ),
            Err(err) => log::warn!(
                "[collect] failed to copy {} -> {}: {err}",
                bbl_source.display(),
                bbl_dest.display()
            ),
        }
    } else {
        log::warn!(
            "[collect] no compiled bibliography found at {}.",
            bbl_source.display()
        );
    }

    rewritten.into_owned()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    fn temp_dir(name: &str) -> std::path::PathBuf {
        let dir =
            std::env::temp_dir().join(format!("texcollector_test_{name}_{}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();
        dir
    }

    #[test]
    fn relocate_strips_leading_parent_dirs_and_copies_file() {
        let root = temp_dir("relocate_basic");
        let input_dir = root.join("input");
        let output_dir = root.join("output");
        fs::create_dir_all(&input_dir).unwrap();
        fs::create_dir_all(&output_dir).unwrap();
        // "../figures/x.pdf" is relative to input_dir, so it resolves to a
        // sibling of input_dir - matching the real fixture layout where
        // figures/ sits alongside pre_print/, not inside it.
        fs::create_dir_all(root.join("figures")).unwrap();
        fs::write(root.join("figures/x.pdf"), b"pdf-bytes").unwrap();

        let relocated = relocate_relative_file("../figures/x.pdf", &input_dir, &output_dir);

        assert_eq!(relocated.as_deref(), Some("figures/x.pdf"));
        assert!(output_dir.join("figures/x.pdf").is_file());
    }

    #[test]
    fn relocate_unescapes_underscore_for_filesystem_lookup_only() {
        let root = temp_dir("relocate_escape");
        let input_dir = root.join("input");
        let output_dir = root.join("output");
        fs::create_dir_all(&input_dir).unwrap();
        fs::create_dir_all(&output_dir).unwrap();
        fs::write(input_dir.join("shap_bar.pdf"), b"pdf-bytes").unwrap();

        let relocated = relocate_relative_file(r"shap\_bar.pdf", &input_dir, &output_dir);

        // The returned path text keeps the LaTeX escape, matching collect.py.
        assert_eq!(relocated.as_deref(), Some(r"shap\_bar.pdf"));
        assert!(output_dir.join("shap_bar.pdf").is_file());
    }

    #[test]
    fn relocate_returns_none_when_source_missing() {
        let root = temp_dir("relocate_missing");
        let input_dir = root.join("input");
        let output_dir = root.join("output");
        fs::create_dir_all(&input_dir).unwrap();
        fs::create_dir_all(&output_dir).unwrap();

        let relocated = relocate_relative_file("does_not_exist.pdf", &input_dir, &output_dir);

        assert!(relocated.is_none());
    }

    #[test]
    fn collect_media_files_rewrites_paths_and_copies() {
        let root = temp_dir("media");
        let input_dir = root.join("input");
        let output_dir = root.join("output");
        fs::create_dir_all(&input_dir).unwrap();
        fs::create_dir_all(&output_dir).unwrap();
        // See note in relocate_strips_leading_parent_dirs_and_copies_file.
        fs::create_dir_all(root.join("figures")).unwrap();
        fs::write(root.join("figures/plot.png"), b"png-bytes").unwrap();

        let source = r"before \includegraphics[width=\linewidth]{../figures/plot.png} after";
        let rewritten = collect_media_files(source, &input_dir, &output_dir);

        assert_eq!(
            rewritten,
            r"before \includegraphics[width=\linewidth]{figures/plot.png} after"
        );
        assert!(output_dir.join("figures/plot.png").is_file());
    }

    #[test]
    fn copy_local_packages_copies_matching_sty_files() {
        let root = temp_dir("packages");
        let input_dir = root.join("input");
        let output_dir = root.join("output");
        fs::create_dir_all(&input_dir).unwrap();
        fs::create_dir_all(&output_dir).unwrap();
        fs::write(input_dir.join("arxiv.sty"), b"sty-bytes").unwrap();

        let source = r"\usepackage{arxiv}";
        let unchanged = copy_local_packages(source, &input_dir, &output_dir);

        assert_eq!(unchanged, source);
        assert!(output_dir.join("arxiv.sty").is_file());
    }

    #[test]
    fn copy_bibliography_rewrites_bib_path_and_copies_bbl() {
        let root = temp_dir("bib");
        let input_dir = root.join("input");
        let output_dir = root.join("output");
        fs::create_dir_all(&input_dir).unwrap();
        fs::create_dir_all(&output_dir).unwrap();
        // "../sources.bib" is relative to input_dir, so it resolves to a
        // sibling of input_dir - matching the real fixture layout.
        fs::write(root.join("sources.bib"), b"@article{}").unwrap();
        fs::write(input_dir.join("main.bbl"), b"\\bibitem{}").unwrap();

        let input_path = input_dir.join("main.tex");
        let output_path = output_dir.join("collected.tex");

        let source = r"\bibliography{../sources}";
        let rewritten = copy_bibliography(source, &input_path, &output_path);

        assert_eq!(rewritten, r"\bibliography{sources}");
        assert!(output_dir.join("sources.bib").is_file());
        assert!(output_dir.join("collected.bbl").is_file());
    }
}
