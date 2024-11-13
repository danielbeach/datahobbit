# datahobbit - CSV or Parquet Generator

A Rust command-line tool that generates CSV or Parquet files with synthetic data based on a provided JSON schema. It supports custom delimiters for CSV, displays a progress bar during generation, and efficiently handles large datasets using parallel processing.

## Table of Contents

- [Features](#features)
- [Installation](#installation)
- [Usage](#usage)
  - [Command-Line Options](#command-line-options)
  - [Schema Definition](#schema-definition)
  - [Examples](#examples)
- [Supported Data Types](#supported-data-types)
- [Contributing](#contributing)
- [License](#license)

## Features

- **Flexible Schema Definition**: Define your data structure using a JSON schema file.
- **Synthetic Data Generation**: Generates realistic data for various data types.
- **CSV and Parquet Support**: Output data in CSV or Parquet format.
- **Parallel Processing**: Utilizes multi-threading for fast data generation.
- **Custom Delimiters**: Supports optional delimiters for CSV, defaulting to a comma.
- **Progress Indicator**: Displays a progress bar during data generation.
- **Error Handling**: Provides clear error messages for unsupported data types or invalid input.

## Installation

To build and run the CSV and Parquet Generator, you need to have [Rust](https://www.rust-lang.org/tools/install) installed on your system.

1. **Clone the Repository**

   ```bash
   git clone https://github.com/yourusername/datahobbit.git
   cd datahobbit
   ```

2. **Build the Project**

   ```bash
   cargo build --release
   ```

   This will create an executable in the `target/release` directory.

## Usage

### Command-Line Options

Run the executable with the following options:

```bash
USAGE:
    datahobbit [OPTIONS] <input> <output>

ARGS:
    <input>     Sets the input JSON schema file
    <output>    Sets the output file (either .csv or .parquet)

OPTIONS:
    -d, --delimiter <DELIMITER>       Sets the delimiter to use in the CSV file (default is ',')
    -h, --help                        Print help information
    -r, --records <RECORDS>           Sets the number of records to generate
    --format <FORMAT>                 Sets the output format: either "csv" or "parquet" (default is "csv")
    --max-file-size <MAX_FILE_SIZE>   Sets the maximum file size for Parquet files in bytes (default is 512 MB)
    -V, --version                     Print version information
```

### Schema Definition

The JSON schema defines the structure of the output file, including column names and data types. Here is an example schema:

```json
{
  "columns": [
    { "name": "id", "type": "integer" },
    { "name": "first_name", "type": "first_name" },
    { "name": "last_name", "type": "last_name" },
    { "name": "email", "type": "email" },
    { "name": "phone_number", "type": "phone_number" },
    { "name": "age", "type": "integer" },
    { "name": "bio", "type": "sentence" },
    { "name": "is_active", "type": "boolean" }
  ]
}
```

### Examples

**Generate a CSV with Default Settings**

```bash
cargo run -- schema.json output.csv --records 100000
```

- Generates 100,000 records.
- Uses the default comma delimiter.

**Generate a Parquet File**

```bash
cargo run -- schema.json output.parquet --records 100000 --format parquet
```

- Generates 100,000 records.
- Outputs data in Parquet format.

**Generate a Parquet File with Custom Size Limit**

```bash
cargo run -- input_schema.json output.parquet --records 1000000 --format parquet --max-file-size 10485760
```
Generates 1,000,000 records.
Outputs data in Parquet format.
Uses a maximum file size of 10 MB, creating additional files as needed.

**Generate a CSV with a Custom Delimiter**

```bash
cargo run -- input_schema.json output.csv --records 100000 --delimiter ';'
```

- Generates 100,000 records.
- Uses a semicolon (`;`) as the delimiter.

**Display Help Information**

```bash
cargo run -- --help
```

## Supported Data Types

The following data types are supported in the schema:

- `integer`: Generates random integers between 0 and 1000.
- `float`: Generates random floating-point numbers between 0.0 and 1000.0.
- `string`: Generates random words.
- `boolean`: Generates random boolean values (`true` or `false`).
- `name`: Generates full names.
- `first_name`: Generates first names.
- `last_name`: Generates last names.
- `email`: Generates email addresses.
- `password`: Generates passwords with lengths between 8 and 16 characters.
- `sentence`: Generates sentences containing 5 to 10 words.
- `phone_number`: Generates phone numbers.

**Example Usage in Schema**

```json
{ "name": "age", "type": "integer" }
{ "name": "description", "type": "sentence" }
{ "name": "is_verified", "type": "boolean" }
```


## License

This project is licensed under the [MIT License](LICENSE).

---

**Author**: Daniel Beach (<dancrystalbeach@gmail.com>)

**Version**: 1.0

---

