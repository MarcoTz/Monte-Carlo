use csv::WriterBuilder;
use std::{error::Error, fs::OpenOptions, path::PathBuf};

pub fn write_csv<T>(data: T, out_path: PathBuf) -> Result<(), Box<dyn Error>>
where
    T: serde::Serialize,
{
    let write_headers = !out_path.exists();
    let file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(out_path)?;
    let mut wtr = WriterBuilder::new()
        .has_headers(write_headers)
        .from_writer(file);
    wtr.serialize(data)?;
    wtr.flush()?;
    Ok(())
}
