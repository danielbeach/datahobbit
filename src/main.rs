use clap::Parser;
use csv_generator::generate_csv;

#[derive(Parser)]
#[command(
    name = "CSV Generator",
    version = "1.0",
    author = "Daniel Beach <dancrystalbeach@gmail.com>",
    about = "Generates CSV files from a JSON schema"
)]
struct Cli {
    /// Sets the input JSON schema file
    input: String,

    /// Sets the output CSV file
    output: String,

    /// Sets the number of records to generate
    #[arg(short, long)]
    records: usize,

    /// Sets the delimiter to use in the CSV file (default is ',')
    #[arg(short, long, default_value = ",")]
    delimiter: char,
}

fn main() {
    let args = Cli::parse();

    // Ensure the delimiter is a single-byte character
    if args.delimiter.len_utf8() != 1 {
        eprintln!("Error: Delimiter must be a single ASCII character.");
        std::process::exit(1);
    }

    if let Err(e) = generate_csv(
        &args.input,
        &args.output,
        args.records,
        args.delimiter as u8,
    ) {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
