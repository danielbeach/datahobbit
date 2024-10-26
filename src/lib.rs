use fake::faker::boolean::en::Boolean;
use fake::faker::internet::en::{Password, SafeEmail};
use fake::faker::lorem::en::{Sentence, Word};
use fake::faker::name::en::{FirstName, LastName, Name};
use rand::Rng;
use fake::faker::phone_number::en::PhoneNumber;
use fake::Fake;
use rayon::iter::{IndexedParallelIterator, IntoParallelIterator, ParallelIterator};
use rayon::prelude::ParallelBridge;
use serde::Deserialize;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::sync::{Arc, Mutex};
use anyhow::{anyhow, Result};

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

pub fn generate_csv(
    input_file: &str,
    output_file: &str,
    records: usize,
) -> anyhow::Result<()> {
    let file = File::open(input_file)?;
    let reader = std::io::BufReader::new(file);
    let schema: Schema = serde_json::from_reader(reader)?;

    let file = File::create(output_file)?;
    let buf_writer = BufWriter::new(file);
    let wtr = csv::Writer::from_writer(buf_writer);
    let wtr = Arc::new(Mutex::new(wtr));

    {
        let headers: Vec<&str> = schema.columns.iter().map(|c| c.name.as_str()).collect();
        let mut wtr = wtr.lock().unwrap();
        wtr.write_record(&headers)?;
    }

    let schema = Arc::new(schema);
    let batch_size = 10_000;

    (0..records)
        .into_par_iter()
        .chunks(batch_size)
        .try_for_each(|chunk| {
            let mut local_records = Vec::with_capacity(batch_size);

            for _ in chunk {
                let mut record = Vec::with_capacity(schema.columns.len());
                for column in &schema.columns {
                    let value = match column.data_type.as_str() {
                        "integer" => rand::thread_rng().gen_range(0..1000).to_string(),
                        "string" => Word().fake::<String>(),
                        "float" => rand::thread_rng().gen_range(0..1000).to_string(),
                        "boolean" => {
                            let bool_value: bool = Boolean(50).fake();
                            bool_value.to_string()
                        }
                        "name" => Name().fake::<String>(),
                        "first_name" => FirstName().fake::<String>(),
                        "last_name" => LastName().fake::<String>(),
                        "email" => SafeEmail().fake::<String>(),
                        "password" => Password(8..16).fake::<String>(),
                        "sentence" => Sentence(5..10).fake::<String>(),
                        "phone_number" => PhoneNumber().fake::<String>(),
                        _ => return Err(anyhow::Error::msg(format!("Unsupported data type: {}", column.data_type))),
                    };
                    record.push(value);
                }
                local_records.push(record);
            }

            let mut wtr = wtr.lock().unwrap();
            for record in local_records {
                wtr.write_record(&record)?;
            }
            Ok(())
        })?;

    let mut wtr = wtr.lock().unwrap();
    wtr.flush()?;
    Ok(())
}
