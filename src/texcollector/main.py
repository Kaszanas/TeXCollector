import logging
from pathlib import Path
import click

from texcollector.settings import LOGGING_FORMAT


def texcollector(input_filepath: Path, output_path: Path, remove_comments: bool = True):
    # TODO: Check if input filepath is valid

    # TODO: open file, read it, parse,

    # TODO: Create output dir if doesn't exist.

    pass


@click.command(help="Tool used for LaTeX source collection.")
@click.option(
    "--input_filepath",
    type=click.Path(exists=True, dir_okay=False, file_okay=True, resolve_path=True),
    required=True,
    help="",
)
@click.option(
    "--output_path",
    type=click.Path(exists=True, dir_okay=True, file_okay=False, resolve_path=True),
    required=True,
    help="",
)
@click.option(
    "--log",
    type=click.Choice(["INFO", "DEBUG", "ERROR"], case_sensitive=False),
    default="WARN",
    help="Log level (INFO, DEBUG, ERROR)",
)
def main(log: str):
    numeric_level = getattr(logging, log.upper(), None)
    if not isinstance(numeric_level, int):
        raise ValueError(f"Invalid log level: {numeric_level}")
    logging.basicConfig(format=LOGGING_FORMAT, level=numeric_level)

    texcollector()


if __name__ == "__main__":
    main()
