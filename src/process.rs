use anyhow::{Ok, Result};
use csv::Reader;
// use serde::{Deserialize, Serialize};
use std::fs;

use crate::options::OutputFormat;

// #[derive(Debug, Deserialize, Serialize)]
// #[serde(rename_all = "PascalCase")]
// struct Record {
//     name: String,
//     position: String,
//     #[serde(rename = "DOB")]
//     dob: String,
//     nationality: String,
//     #[serde(rename = "Kit Number")]
//     kot: u8,
// }

pub fn process_csv(input: &str, output: String, format: OutputFormat) -> Result<()> {
    let mut reader = Reader::from_path(input)?;
    let mut writer = Vec::with_capacity(128);
    let headers = reader.headers()?.clone();
    for result in reader.records() {
        let record = result?;
        // headers.iter() -> 使用headers 的迭代器
        // record.iter() -> 使用 record 的迭代器
        // zip -> 合并两个迭代器的元素，形成一个元组 [(header, record),..]
        // .zip(record.iter()) -> 将headers和record中的值组合成元组
        // .collect() -> 将元组收集到一个新的Vec中
        // collect::<Value>() -> 将元组的迭代器转换为 JSON Value
        let json_value = headers
            .iter()
            .zip(record.iter())
            .collect::<serde_json::Value>();

        writer.push(json_value);
    }
    let content = match format {
        OutputFormat::Json => serde_json::to_string_pretty(&writer)?,
        OutputFormat::Yaml => serde_yaml::to_string(&writer)?,
    };
    fs::write(output, content)?;
    Ok(())
}
