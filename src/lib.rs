use anyhow::{anyhow, Result};
use fake::Fake;
use fake::faker::boolean::en::Boolean;
use fake::faker::internet::en::{Password, SafeEmail};
use fake::faker::lorem::en::{Sentence, Word};
use fake::faker::name::en::{FirstName, LastName, Name};
use fake::faker::phone_number::en::PhoneNumber;
use indicatif::{ProgressBar, ProgressStyle};
use rand::Rng;
use rayon::iter::{IndexedParallelIterator, IntoParallelIterator, ParallelIterator};
use rayon::prelude::ParallelBridge;
use serde::Deserialize;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::sync::{Arc, Mutex};
use arrow::array::{ArrayRef, BooleanArray, Float64Array, Int64Array, StringArray};
use arrow::record_batch::RecordBatch;
use parquet::arrow::arrow_writer::ArrowWriter;
use parquet::file::properties::WriterProperties;
use parquet::file::writer::SerializedFileWriter;
use parquet::schema::types::Type;

#[derive(Deserialize)]
struct Schema {
    columns: Vec<Column>,
}

#[derive(Deserialize)]
struct Column {
    name: String,
    #[serde(rename = "type")]
    data_type: String,
}

fn get_generator(data_type: &str) -> Result<Box<dyn Fn() -> String + Send + Sync>> {
    match data_type {
        "integer" => Ok(Box::new(|| rand::thread_rng().gen_range(0..1000).to_string())),
        "string" => Ok(Box::new(|| Word().fake::<String>())),
        "float" => Ok(Box::new(|| rand::thread_rng().gen_range(0.0..1000.0).to_string())),
        "boolean" => Ok(Box::new(|| Boolean(50).fake::<bool>().to_string())),
        "name" => Ok(Box::new(|| Name().fake::<String>())),
        "first_name" => Ok(Box::new(|| FirstName().fake::<String>())),
        "last_name" => Ok(Box::new(|| LastName().fake::<String>())),
        "email" => Ok(Box::new(|| SafeEmail().fake::<String>())),
        "password" => Ok(Box::new(|| Password(8..16).fake::<String>())),
        "sentence" => Ok(Box::new(|| Sentence(5..10).fake::<String>())),
        "phone_number" => Ok(Box::new(|| PhoneNumber().fake::<String>())),
        _ => Err(anyhow!("Unsupported data type: {}", data_type)),
    }
}

pub fn generate_parquet(input_file: &str, output_file: &str, records: usize) -> Result<()> {
    let file = File::open(input_file)?;
    let reader = std::io::BufReader::new(file);
    let schema: Schema = serde_json::from_reader(reader)?;

    let arrow_fields = schema.columns.iter()
        .map(|col| match col.data_type.as_str() {
            "integer" => Ok(arrow::datatypes::Field::new(&col.name, arrow::datatypes::DataType::Int64, false)),
            "float" => Ok(arrow::datatypes::Field::new(&col.name, arrow::datatypes::DataType::Float64, false)),
            "boolean" => Ok(arrow::datatypes::Field::new(&col.name, arrow::datatypes::DataType::Boolean, false)),
            "string" | "name" | "first_name" | "last_name" | "email" | "password" | "sentence" | "phone_number" => 
                Ok(arrow::datatypes::Field::new(&col.name, arrow::datatypes::DataType::Utf8, false)),
            _ => Err(anyhow!("Unsupported data type for Parquet: {}", col.data_type)),
        })
        .collect::<Result<Vec<_>>>()?;

    let arrow_schema = Arc::new(arrow::datatypes::Schema::new(arrow_fields));
    let file = File::create(output_file)?;
    let buf_writer = BufWriter::new(file);
    let writer_props = WriterProperties::builder().build();
    let mut writer = ArrowWriter::try_new(buf_writer, arrow_schema.clone(), Some(writer_props))?;

    let generators: Vec<_> = schema.columns.iter()
        .map(|column| get_generator(&column.data_type))
        .collect::<Result<_>>()?;

    let mut batches = Vec::new();

    for _ in 0..records {
        let row: Vec<String> = generators.iter().map(|gen| gen()).collect();
        let columns: Vec<ArrayRef> = row.iter().enumerate().map(|(i, value)| {
            match schema.columns[i].data_type.as_str() {
                "integer" => Arc::new(Int64Array::from(vec![value.parse::<i64>().unwrap()])) as ArrayRef,
                "float" => Arc::new(Float64Array::from(vec![value.parse::<f64>().unwrap()])) as ArrayRef,
                "boolean" => Arc::new(BooleanArray::from(vec![value == "true"])) as ArrayRef,
                _ => Arc::new(StringArray::from(vec![value.to_string()])) as ArrayRef,
            }
        }).collect();

        let batch = RecordBatch::try_new(arrow_schema.clone(), columns)?;
        batches.push(batch);
    }

    for batch in batches {
        writer.write(&batch)?;
    }

    writer.close()?;
    Ok(())
}


pub fn generate_csv(
    input_file: &str,
    output_file: &str,
    records: usize,
    delimiter: u8,
) -> Result<()> {
    let file = File::open(input_file)?;
    let reader = std::io::BufReader::new(file);
    let schema: Schema = serde_json::from_reader(reader)?;

    let file = File::create(output_file)?;
    let buf_writer = BufWriter::new(file);

    // Create CSV writer with the specified delimiter
    let mut csv_writer_builder = csv::WriterBuilder::new();
    csv_writer_builder.delimiter(delimiter);
    let wtr = csv_writer_builder.from_writer(buf_writer);
    let wtr = Arc::new(Mutex::new(wtr));

    {
        let headers: Vec<&str> = schema.columns.iter().map(|c| c.name.as_str()).collect();
        let mut wtr = wtr.lock().unwrap();
        wtr.write_record(&headers)?;
    }

    let generators: Vec<_> = schema
        .columns
        .iter()
        .map(|column| get_generator(&column.data_type))
        .collect::<Result<_>>()?;

    let batch_size = 10_000;

    // Create a progress bar
    let progress_bar = Arc::new(ProgressBar::new(records as u64));

    // Handle the Result from .template()
    let progress_style = ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {percent}% ({pos}/{len})")
        .expect("Failed to set progress bar template")
        .progress_chars("#>-");
    progress_bar.set_style(progress_style);

    // Start generating records in parallel
    (0..records)
        .into_par_iter()
        .chunks(batch_size)
        .try_for_each(|chunk| -> Result<()> {
            let mut local_records = Vec::with_capacity(batch_size);

            for _ in chunk {
                let record: Vec<String> = generators.iter().map(|gen| gen()).collect();
                local_records.push(record);
            }

            {
                let mut wtr = wtr.lock().unwrap();
                for record in &local_records {
                    wtr.write_record(record)?;
                }
            }

            // Update the progress bar
            progress_bar.inc(local_records.len() as u64);

            Ok(())
        })?;

    progress_bar.finish_with_message("Data generation complete.");

    let mut wtr = wtr.lock().unwrap();
    wtr.flush()?;
    Ok(())
}