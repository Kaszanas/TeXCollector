use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{bail, Context, Result};
use clap::Parser;
use simple_logger::SimpleLogger;
use texcollector::cli;
use texcollector::collector;

fn parent_dir(path: &Path) -> &Path {
    match path.parent() {
        Some(parent) if !parent.as_os_str().is_empty() => parent,
        _ => Path::new("."),
    }
}

/// Input/output paths validated and resolved before any collector step runs.
struct ResolvedPaths {
    /// Canonicalized `main.tex` path.
    input_path: PathBuf,
    /// Parent directory of `input_path`.
    input_dir: PathBuf,
    /// Parent directory of the output file, created if missing.
    output_dir: PathBuf,
}

/// Validate and resolve the input/output paths before any collector step runs.
///
/// `input_path` is canonicalized so its parent directory is always real and
/// non-empty — a bare relative filename like `main.tex` otherwise yields an
/// empty `Path::parent()`, which breaks `latexpand` (it needs a real CWD to
/// resolve `\input`/`\include` against). `output_path` isn't canonicalized
/// since it may not exist yet; its parent directory is created instead.
fn resolve_paths(input_path: &Path, output_path: &Path) -> Result<ResolvedPaths> {
    if !input_path.is_file() {
        bail!("main file not found: {}", input_path.display());
    }

    let input_path = fs::canonicalize(input_path)
        .with_context(|| format!("failed to resolve {}", input_path.display()))?;
    let input_dir = input_path
        .parent()
        .with_context(|| format!("{} has no parent directory", input_path.display()))?
        .to_path_buf();

    let output_dir = parent_dir(output_path).to_path_buf();
    fs::create_dir_all(&output_dir)
        .with_context(|| format!("failed to create output directory {}", output_dir.display()))?;

    Ok(ResolvedPaths {
        input_path,
        input_dir,
        output_dir,
    })
}

fn main() -> Result<()> {
    SimpleLogger::new().init().unwrap();

    log::info!("Initialized the program!");

    let args = cli::CLIArguments::parse();
    let output_path = args.output_path;

    let resolved = resolve_paths(&args.main_file, &output_path)?;

    log::info!(
        "Set LaTeX main file to: {}",
        resolved.input_path.to_string_lossy()
    );
    log::info!(
        "Set output path for the collected files to: {}",
        output_path.to_string_lossy()
    );

    // Run the collector functions in sequence, passing the output of one as the input to the next:
    let expanded = collector::call_latexpand(&resolved.input_path)?;
    let expanded =
        collector::collect_media_files(&expanded, &resolved.input_dir, &resolved.output_dir);
    let expanded =
        collector::copy_local_packages(&expanded, &resolved.input_dir, &resolved.output_dir);
    let expanded = collector::copy_bibliography(&expanded, &resolved.input_path, &output_path);

    // Finally, save the expanded LaTeX file to the output path:
    collector::save_to_file(&expanded, &output_path)?;

    // Return success:
    Ok(())
}
