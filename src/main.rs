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
}

fn main() {
    let args = Cli::parse();

    if let Err(e) = generate_csv(&args.input, &args.output, args.records) {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
