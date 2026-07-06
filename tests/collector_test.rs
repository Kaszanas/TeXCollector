//! Integration test running the full collector pipeline against the fixture
//! project at `tests/files/input/pre_print/main.tex`, mirroring what
//! `python_example/collect.py` does against the same kind of project.
//!
//! Requires the `latexpand` binary on `PATH` (same requirement as the
//! Python reference script). The test is skipped with a message rather
//! than failing when `latexpand` isn't available.

use std::fs;
use std::path::PathBuf;
use std::process::Command;

use texcollector::collector;

fn latexpand_available() -> bool {
    Command::new("latexpand").arg("--version").output().is_ok()
}

#[test]
fn collects_fixture_project_into_self_contained_bundle() {
    if !latexpand_available() {
        eprintln!("skipping: `latexpand` not found on PATH");
        return;
    }

    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let input_path = manifest_dir.join("tests/files/input/pre_print/main.tex");

    let output_dir = std::env::temp_dir().join(format!(
        "texcollector_collector_test_{}",
        std::process::id()
    ));
    let _ = fs::remove_dir_all(&output_dir);
    fs::create_dir_all(&output_dir).unwrap();
    let output_path = output_dir.join("collected.tex");

    let input_dir = input_path.parent().unwrap();

    let expanded = collector::call_latexpand(&input_path).expect("latexpand should succeed");
    let expanded = collector::collect_media_files(&expanded, input_dir, &output_dir);
    let expanded = collector::copy_local_packages(&expanded, input_dir, &output_dir);
    let expanded = collector::copy_bibliography(&expanded, &input_path, &output_path);
    collector::save_to_file(&expanded, &output_path).expect("saving output should succeed");

    assert!(!expanded.contains(r"\includegraphics{../"));
    assert!(!expanded.contains(r"\bibliography{../"));

    assert!(output_dir.join("figures/picture.png").is_file());
    assert!(output_dir.join("arxiv.sty").is_file());
    assert!(output_dir.join("sources.bib").is_file());
    assert!(output_dir.join("collected.bbl").is_file());
}
