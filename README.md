# DataHobbit

A Rust-based CLI application to generate CSV files with realistic fake data based on a user-defined JSON schema. The application supports multithreaded data generation, and batching.

## Features

- **Customizable Schema**: Define the structure and data types of your CSV file using a JSON schema.
- **Realistic Fake Data**: Generates realistic data using the [`fake`](https://crates.io/crates/fake) crate.
- **Multithreaded Processing**: Utilizes multiple CPU cores for faster data generation with the [`rayon`](https://crates.io/crates/rayon) crate.
- **Batch Writing**: Reduces lock contention by writing data in batches.
- **Progress Bar**: Displays a progress bar during execution using the [`indicatif`](https://crates.io/crates/indicatif) crate.
- **Error Handling**: Provides detailed error messages for unsupported data types and other issues.

## Table of Contents

- [Prerequisites](#prerequisites)
- [Installation](#installation)
- [Usage](#usage)
  - [Creating a JSON Schema](#creating-a-json-schema)
  - [Running the Application](#running-the-application)
- [Examples](#examples)
  - [Basic Example](#basic-example)
  - [Extended Schema Example](#extended-schema-example)
- [Customization](#customization)
  - [Adding New Data Types](#adding-new-data-types)
  - [Adjusting Batch Size](#adjusting-batch-size)
- [Performance Considerations](#performance-considerations)
- [Troubleshooting](#troubleshooting)
- [License](#license)

## Prerequisites

- **Rust Toolchain**: Ensure you have Rust and Cargo installed. You can install them using [Rustup](https://rustup.rs/).

  ```bash
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  ```

## Installation

1. **Clone the Repository**

   ```bash
   git clone https://github.com/yourusername/csv_generator.git
   cd csv_generator
   ```

2. **Update Dependencies**

   Ensure all dependencies are up-to-date:

   ```bash
   cargo update
   ```

3. **Build the Project**

   Build the application in release mode for optimal performance:

   ```bash
   cargo build --release
   ```

   The compiled binary will be located at `./target/release/csv_generator`.

## Usage

### Creating a JSON Schema

Create a JSON file (e.g., `schema.json`) that defines the structure and data types of the CSV file you want to generate.

**Supported Data Types:**

- `integer`
- `float`
- `string`
- `boolean`
- `name`
- `first_name`
- `last_name`
- `email`
- `password`
- `sentence`
- `phone_number`

**Example `schema.json`:**

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

### Running the Application

Use the compiled binary to generate the CSV file.

**Command Syntax:**

```bash
./target/release/csv_generator <schema_file> <output_csv_file> --records <number_of_records>
```

**Example:**

```bash
./target/release/csv_generator schema.json output.csv --records 100000
```

- `schema.json`: Path to your JSON schema file.
- `output.csv`: Desired name for the output CSV file.
- `100000`: Number of records to generate.

## Examples

### Basic Example

Generate a CSV file with 10,000 records using the basic schema:

```bash
./target/release/csv_generator schema.json output.csv --records 10000
```

### Extended Schema Example

**Extended `schema.json`:**

```json
{
  "columns": [
    { "name": "id", "type": "integer" },
    { "name": "full_name", "type": "name" },
    { "name": "username", "type": "username" },
    { "name": "email", "type": "email" },
    { "name": "city", "type": "city" },
    { "name": "country", "type": "country" },
    { "name": "postal_code", "type": "postal_code" },
    { "name": "company", "type": "company" },
    { "name": "job_title", "type": "job_title" },
    { "name": "registration_date", "type": "date" }
  ]
}
```

**Note:** You'll need to update the code to handle these additional data types. See [Customization](#customization) for details.

**Generate CSV:**

```bash
./target/release/csv_generator extended_schema.json extended_output.csv --records 50000
```

## Customization

### Adding New Data Types

To support additional data types:

1. **Import the Necessary Fakers**

   In `src/lib.rs`, import the required fakers from the `fake` crate.

   ```rust
   use fake::faker::address::raw::{CityName, CountryName, PostCode};
   use fake::faker::company::raw::{CompanyName, JobTitle};
   use fake::faker::internet::raw::Username;
   use fake::faker::chrono::raw::DateTimeBetween;
   use chrono::NaiveDate;
   ```

2. **Update the Match Statement**

   Add cases for the new data types:

   ```rust
   match column.data_type.as_str() {
       // Existing cases...
       "username" => Username().fake::<String>(),
       "city" => CityName().fake::<String>(),
       "country" => CountryName().fake::<String>(),
       "postal_code" => PostCode().fake::<String>(),
       "company" => CompanyName().fake::<String>(),
       "job_title" => JobTitle().fake::<String>(),
       "date" => DateTimeBetween(
                   Some(NaiveDate::from_ymd(2000, 1, 1).and_hms(0, 0, 0)),
                   Some(chrono::Local::now().naive_local())
               ).fake::<chrono::NaiveDateTime>().to_string(),
       // ...
   }
   ```

3. **Rebuild the Project**

   ```bash
   cargo build --release
   ```

### Adjusting Batch Size

The batch size determines how many records each thread processes before writing to the CSV file.

- **Default Batch Size:** `10,000`
- **Adjusting Batch Size:**

  In `src/lib.rs`, modify the `batch_size` variable:

  ```rust
  let batch_size = 5000; // For smaller batches
  ```

- **Considerations:**
  - Smaller batches may provide more frequent progress updates but can increase lock contention.
  - Larger batches reduce lock contention but increase memory usage.

## Performance Considerations

- **Multithreading:**
  - The application uses all available CPU cores to speed up data generation.
  - The `rayon` crate automatically manages thread pooling.

- **Memory Usage:**
  - Be mindful of the memory requirements, especially with large batch sizes or a high number of records.

- **I/O Bottlenecks:**
  - Disk write speed can become a bottleneck. Using an SSD can improve performance.

## Troubleshooting

- **Compilation Errors:**
  - Ensure all dependencies are correctly specified in `Cargo.toml`.
  - Run `cargo update` to fetch the latest versions.

- **Unsupported Data Types:**
  - If you receive an error about an unsupported data type, ensure you've imported the necessary fakers and updated the match statement.

- **Order of Records:**
  - The order of records in the CSV file may not be sequential due to multithreading. If order is important, additional logic is needed.

## License

This project is licensed under the MIT License.
---

Feel free to contribute to this project by submitting issues or pull requests. If you have any questions or need assistance, please open an issue on the repository.
