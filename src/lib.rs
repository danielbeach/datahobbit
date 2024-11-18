mod csv_generators;
use crate::csv_generators::{generate_csv, generate_parquet};

use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

#[pyfunction]
fn py_entry_point(
    input: &str,
    output: &str,
    records: usize,
    delimiter: u8,
    data_format: &str,
    max_file_size: usize,
) -> PyResult<()> {
    let result = match data_format {
        "csv" => generate_csv(&input, &output, records, delimiter),
        "parquet" => generate_parquet(&input, &output, records, max_file_size),
        _ => unreachable!(),
    };

    match result {
        Err(e) => Err(PyErr::new::<PyValueError, _>(format!("Error: {}", e))),
        Ok(_) => Ok(()),
    }
}

#[pymodule]
fn native(_py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(py_entry_point, m)?)?;

    Ok(())
}
