use clap::Parser;
use csv_generator::{generate_csv, generate_parquet};

#[derive(Parser)]
#[command(
    name = "Data Generator",
    version = "1.0",
    author = "Daniel Beach <dancrystalbeach@gmail.com>",
    about = "Generates CSV or Parquet files from a JSON schema"
)]
struct Cli {
    /// Sets the input JSON schema file
    input: String,

    /// Sets the output file
    output: String,

    /// Sets the number of records to generate
    #[arg(short, long)]
    records: usize,

    /// Sets the delimiter to use in the CSV file (default is ',')
    #[arg(short, long, default_value = ",")]
    delimiter: char,

    /// Output format: either "csv" or "parquet"
    #[arg(long, default_value = "csv")]
    format: String,
}

fn main() {
    let args = Cli::parse();

    // Ensure the delimiter is a single-byte character for CSV
    if args.format == "csv" && args.delimiter.len_utf8() != 1 {
        eprintln!("Error: Delimiter must be a single ASCII character.");
        std::process::exit(1);
    }

    let result = match args.format.as_str() {
        "csv" => generate_csv(&args.input, &args.output, args.records, args.delimiter as u8),
        "parquet" => generate_parquet(&args.input, &args.output, args.records),
        _ => Err(anyhow::anyhow!("Unsupported format: {}", args.format)),
    };

    if let Err(e) = result {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
