use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{bail, Context, Result};
use clap::Parser;
use log::info;
use simple_logger::SimpleLogger;
use texcollector::cli;
use texcollector::collector;

/// Input/output paths validated and resolved before any collector step runs.
struct ResolvedPaths {
    /// Canonicalized `main.tex` path.
    input_path: PathBuf,
    /// Parent directory of `input_path`.
    input_dir: PathBuf,
    /// Canonicalized output directory, created if missing.
    output_dir: PathBuf,
    /// `output_dir` joined with the output file name — the final `.tex` destination.
    output_file_path: PathBuf,
}

/// Validate and resolve the input/output paths before any collector step runs.
///
/// `input_path` is canonicalized so its parent directory is always real and
/// non-empty — a bare relative filename like `main.tex` otherwise yields an
/// empty `Path::parent()`, which breaks `latexpand` (it needs a real CWD to
/// resolve `\input`/`\include` against). `output_dir_arg` is a directory that
/// may not exist yet, so it's created before being canonicalized.
fn resolve_paths(
    input_path: &Path,
    output_dir_arg: &Path,
    output_file_name: &str,
) -> Result<ResolvedPaths> {
    if !input_path.is_file() {
        bail!("main file not found: {}", input_path.display());
    }

    let input_path = fs::canonicalize(input_path)
        .with_context(|| format!("failed to resolve {}", input_path.display()))?;

    info!("Resolved input path to: {}", input_path.display());

    let input_dir = input_path
        .parent()
        .with_context(|| format!("{} has no parent directory", input_path.display()))?
        .to_path_buf();

    info!("Resolved input directory to: {}", input_dir.display());

    fs::create_dir_all(output_dir_arg).with_context(|| {
        format!(
            "failed to create output directory {}",
            output_dir_arg.display()
        )
    })?;

    let output_dir = fs::canonicalize(output_dir_arg)
        .with_context(|| format!("failed to resolve {}", output_dir_arg.display()))?;

    info!("Resolved output directory to: {}", output_dir.display());

    let output_file_path = output_dir.join(output_file_name);

    info!(
        "Resolved output file path to: {}",
        output_file_path.display()
    );

    Ok(ResolvedPaths {
        input_path,
        input_dir,
        output_dir,
        output_file_path,
    })
}

fn main() -> Result<()> {
    SimpleLogger::new().init().unwrap();

    let args = cli::CLIArguments::parse();

    log::info!("Initialized the program!");

    let resolved = resolve_paths(&args.input_file, &args.output_path, &args.output_file_name)?;

    // Run the collector functions in sequence, passing the output of one as the input to the next:
    let expanded = collector::call_latexpand(&resolved.input_path)?;
    let expanded =
        collector::collect_media_files(&expanded, &resolved.input_dir, &resolved.output_dir);
    let expanded =
        collector::copy_local_packages(&expanded, &resolved.input_dir, &resolved.output_dir);
    let expanded =
        collector::copy_bibliography(&expanded, &resolved.input_path, &resolved.output_file_path);

    // Finally, save the expanded LaTeX file to the output path:
    collector::save_to_file(&expanded, &resolved.output_file_path)?;

    // Return success:
    Ok(())
}
