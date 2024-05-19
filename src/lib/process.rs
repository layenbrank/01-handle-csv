use anyhow::{Ok, Result};
use csv::Reader;
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
struct Record {
    name: String,
    position: String,
    #[serde(rename = "DOB")]
    dob: String,
    nationality: String,
    #[serde(rename = "Kit Number")]
    kot: u8,
}

pub fn process_csv(input: &str, output: &str) -> Result<()> {
    let mut reader = Reader::from_path(input)?;
    let mut writer = Vec::with_capacity(128);
    for result in reader.deserialize() {
        let record: Record = result?;
        writer.push(record);
    }
    let json = serde_json::to_string_pretty(&writer)?;
    fs::write(output, json)?;
    Ok(())
}
