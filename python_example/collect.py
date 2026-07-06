import argparse
import re
import shutil
import subprocess
from pathlib import Path

INCLUDEGRAPHICS_RE = re.compile(r"\\includegraphics(\[[^\]]*\])?\{([^}]*)\}")
USEPACKAGE_RE = re.compile(r"\\usepackage(?:\[[^\]]*\])?\{([^}]*)\}")
BIBLIOGRAPHY_RE = re.compile(r"\\bibliography\{([^}]*)\}")
LEADING_PARENT_RE = re.compile(r"^(?:\.\./)+")


def call_latexpand(input_path: Path) -> str:
    # latexpand resolves \input/\include relative to its CWD, not the
    # location of the file being processed, so run it from there.
    result = subprocess.run(
        ["latexpand", input_path.name],
        capture_output=True,
        text=True,
        check=True,
        cwd=input_path.parent,
    )

    # Return the output of the command
    return result.stdout


def save_to_file(output: str, filename: str) -> None:
    with open(filename, "w") as f:
        f.write(output)


def _relocate_relative_file(
    raw_path: str, input_dir: Path, output_dir: Path
) -> str | None:
    """Copy a single \\-escaped relative path referenced from input_dir into
    output_dir, stripping any leading "../" so the result no longer climbs
    out of output_dir (e.g. "../figures_tables/x.pdf" -> "figures_tables/x.pdf").

    Returns the rewritten relative path text on success, or None if the
    source file doesn't exist (a warning is printed in that case).
    """
    leading = LEADING_PARENT_RE.match(raw_path)
    remainder = raw_path[leading.end() :] if leading else raw_path

    # LaTeX filenames sometimes escape underscores (e.g. "shap\_bar.pdf")
    # even though the file on disk has a literal underscore.
    source = (input_dir / raw_path.replace("\\_", "_")).resolve()
    delatexed_remainder = remainder.replace("\\_", "_")

    if not source.is_file():
        print(f"[collect] WARNING: file not found, leaving path as-is: {raw_path}")
        return None

    dest = output_dir / delatexed_remainder
    dest.parent.mkdir(parents=True, exist_ok=True)
    shutil.copy2(source, dest)

    return remainder


def collect_media_files(tex_source: str, input_dir: Path, output_dir: Path) -> str:
    """Copy every \\includegraphics target referenced in tex_source from
    input_dir into output_dir, rewriting paths so they no longer climb out
    of output_dir (e.g. "../figures_tables/x.pdf" -> "figures_tables/x.pdf").
    """
    copied = 0
    warned = 0

    def replace(match: re.Match) -> str:
        nonlocal copied, warned

        options, raw_path = match.group(1) or "", match.group(2)
        relocated = _relocate_relative_file(raw_path, input_dir, output_dir)

        if relocated is None:
            warned += 1
            return match.group(0)

        copied += 1
        return f"\\includegraphics{options}{{{relocated}}}"

    rewritten = INCLUDEGRAPHICS_RE.sub(replace, tex_source)
    print(f"[collect] Copied {copied} figure(s), {warned} warning(s).")
    return rewritten


def copy_local_packages(tex_source: str, input_dir: Path, output_dir: Path) -> str:
    """Copy any local .sty file referenced via \\usepackage{...} that lives
    alongside the main .tex file into output_dir. Package names don't
    encode a path, so no text rewriting is needed.
    """
    copied = []

    for match in USEPACKAGE_RE.finditer(tex_source):
        for name in match.group(1).split(","):
            name = name.strip()
            source = input_dir / f"{name}.sty"
            if source.is_file():
                shutil.copy2(source, output_dir / source.name)
                copied.append(source.name)

    print(
        f"[collect] Copied {len(copied)} local package(s): {', '.join(copied) or '-'}."
    )
    return tex_source


def copy_bibliography(tex_source: str, input_path: Path, output_path: Path) -> str:
    """Copy any locally-resolvable \\bibliography{...} entries (e.g.
    "../sources.bib") into the bundle and rewrite their path, leaving
    distribution-provided entries (e.g. "IEEEabrv") untouched. Also copies
    the already-built <input stem>.bbl, renamed to match the output stem,
    so the bibliography compiles without re-running bibtex.
    """
    input_dir = input_path.parent
    output_dir = output_path.parent

    def replace(match: re.Match) -> str:
        entries = [entry.strip() for entry in match.group(1).split(",")]
        rewritten_entries = []

        for entry in entries:
            has_extension = entry.endswith(".bib")
            raw_path = entry if has_extension else f"{entry}.bib"
            relocated = _relocate_relative_file(raw_path, input_dir, output_dir)

            if relocated is None:
                rewritten_entries.append(entry)
            else:
                rewritten_entries.append(
                    relocated if has_extension else relocated[: -len(".bib")]
                )

        return f"\\bibliography{{{','.join(rewritten_entries)}}}"

    rewritten = BIBLIOGRAPHY_RE.sub(replace, tex_source)

    bbl_source = input_path.with_suffix(".bbl")
    if bbl_source.is_file():
        shutil.copy2(bbl_source, output_path.with_suffix(".bbl"))
        print(
            f"[collect] Copied bibliography: {bbl_source.name} -> {output_path.with_suffix('.bbl').name}."
        )
    else:
        print(f"[collect] WARNING: no compiled bibliography found at {bbl_source}.")

    return rewritten


if __name__ == "__main__":
    repo_root = Path(__file__).parent

    parser = argparse.ArgumentParser(
        description=(
            "Flatten the article's main .tex file with latexpand and collect "
            "the figures it references into a self-contained arXiv submission "
            "bundle (relative \\includegraphics paths are rewritten to match "
            "the new bundle layout)."
        )
    )
    parser.add_argument(
        "--input_file",
        type=str,
        help="Path to the main latex file to flatten.",
        default=str(repo_root / "src" / "pre_print" / "latent_trainer.tex"),
    )
    parser.add_argument(
        "--output_file",
        type=str,
        help="Path to write the flattened latex file to.",
        default=str(repo_root / "arxiv_submission" / "collected.tex"),
    )

    args = parser.parse_args()

    input_path = Path(args.input_file).resolve()
    output_path = Path(args.output_file).resolve()
    input_dir = input_path.parent
    output_dir = output_path.parent
    output_dir.mkdir(parents=True, exist_ok=True)

    # Call latexpand and capture the output
    expanded_output = call_latexpand(input_path)

    # Relocate referenced figures alongside the output and fix up their paths
    expanded_output = collect_media_files(expanded_output, input_dir, output_dir)

    # Copy local .sty packages referenced via \usepackage
    expanded_output = copy_local_packages(expanded_output, input_dir, output_dir)

    # Copy the bibliography source and the already-compiled .bbl
    expanded_output = copy_bibliography(expanded_output, input_path, output_path)

    # Save the expanded output to a file
    save_to_file(expanded_output, str(output_path))
