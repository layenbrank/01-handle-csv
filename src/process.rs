use anyhow::Result;
use csv::Reader;
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct Record {
    name: String,
    position: String,
    #[serde(rename = "DOB")]
    dob: String,
    nationality: String,
    #[serde(rename = "Kit Number")]
    kit: u8,
}

pub fn process_csv(input: &str, output: &str) -> Result<()> {
    let mut reader = Reader::from_path(input)?;
    let mut results = Vec::with_capacity(128);
    for result in reader.deserialize() {
        let record: Record = result?;
        results.push(record);
    }

    let json = serde_json::to_string_pretty(&results)?;
    fs::write(output, json)?;

    // let records = reader
    //     .deserialize()
    //     .map(|record| record.unwrap())
    //     .collect::<Vec<Record>>();
    // println!("{:?}", records);

    // let file = File::open(&opts.input).unwrap();
    // let reader = BufReader::new(file);
    // let mut csv_reader = Reader::from_reader(reader);
    // let mut records = Vec::new();
    // for result in csv_reader.records() {
    //     let record = result.unwrap();
    //     records.push(record);
    // }
    // println!("{:?}", records);
    Ok(())
}
