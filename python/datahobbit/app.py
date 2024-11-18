from enum import Enum

import typer
from typing_extensions import Annotated

from .native import py_entry_point

NATIVE_USIZE_MAX_VALUE = 18_446_744_073_709_551_615


help_str = """
[bold][green]Data Generator[/green][/bold]\n
[italic]Generates CSV or Parquet files from a JSON schema[/italic]

Author: Daniel Beach <dancrystalbeach@gmail.com>
Source code: https://github.com/danielbeach/datahobbit
"""

app = typer.Typer(
    rich_markup_mode="rich",
    help=help_str,
)


class FormatType(str, Enum):
    CSV = "csv"
    PARQUET = "parquet"


def main(
    input: Annotated[str, typer.Option(help="Sets the input JSON schema file")],
    output: Annotated[str, typer.Option(help="Sets the output file prefix")],
    records: Annotated[
        int, typer.Option(min=0, max=NATIVE_USIZE_MAX_VALUE, help="Sets the number of records to generate")
    ],
    delimeter: Annotated[str, typer.Option(help="Sets the delimiter to use in the CSV file (default is ',')")] = ",",
    format: Annotated[
        FormatType, typer.Option("--format", help='Output format: either "csv" or "parquet"')
    ] = FormatType.CSV,
    max_file_size: Annotated[
        int,
        typer.Option(min=0, max=NATIVE_USIZE_MAX_VALUE, help="Sets the maximum file size for Parquet files (in bytes)"),
    ] = 100 * 1024 * 1024,
):
    if len(delimeter) != 1:
        raise ValueError(f"Delimeter expected to be a one symbol, but got '{delimeter}'")
    else:
        delimeter_char = ord(delimeter)
    py_entry_point(input, output, records, delimeter_char, format.value, max_file_size)


def entry_point():
    typer.run(main)


if __name__ == "__main__":
    entry_point()
