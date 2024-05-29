use base64::{
    engine::general_purpose::{STANDARD, URL_SAFE},
    Engine as _,
};
use std::{fs::File, io::Read};

use crate::Base64Format;

pub fn process_encode(input: &str, format: Base64Format) -> anyhow::Result<()> {
    let mut reader: Box<dyn Read> = if input == "-" {
        Box::new(std::io::stdin())
    } else {
        Box::new(File::open(input)?)
    };
    let mut buffer = Vec::new();
    reader.read_to_end(&mut buffer)?;

    let encoded = match format {
        Base64Format::Standard => STANDARD.encode(&buffer),
        Base64Format::UrlSafe => URL_SAFE.encode(&buffer),
    };
    println!("{:?}", encoded);
    Ok(())
}
pub fn process_decode(input: &str, format: Base64Format) -> anyhow::Result<()> {
    let mut reader: Box<dyn Read> = if input == "-" {
        Box::new(std::io::stdin())
    } else {
        Box::new(File::open(input)?)
    };
    let mut buffer = Vec::new();
    reader.read_to_end(&mut buffer)?;

    let decoded = match format {
        Base64Format::Standard => STANDARD.decode(&buffer),
        Base64Format::UrlSafe => URL_SAFE.decode(&buffer),
    };
    println!("{:?}", decoded);
    Ok(())
}
