mod base64;
mod csv;
mod genpass;

use std::path::Path;

use clap::Parser;

pub use self::{
    base64::{Base64Format, Base64Options},
    csv::OutputFormat,
};

use self::{csv::CsvOptions, genpass::GeneratePasswordOptions};

#[derive(Debug, Parser)]
#[command(name="csvtool", version, author, about, long_about = None)]
pub struct Options {
    #[command(subcommand)]
    pub cmd: SubCommand,
}

#[derive(Debug, Parser)]
pub enum SubCommand {
    #[command(name = "csv", about = "Show CSV, or Convert CSV to other formats")]
    Csv(CsvOptions),
    #[command(name = "generatePassword", about = "Generate password")]
    GeneratePassword(GeneratePasswordOptions),
    #[command(subcommand)]
    Base64(Base64Options),
}

fn verify_input_file(filename: &str) -> Result<String, &'static str> {
    // if input is "-" or file exits
    if filename == "-" || Path::new(filename).exists() {
        Ok(filename.into())
    } else {
        Err("File not found")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_verify_input_file() {
        assert!(verify_input_file("-").is_ok());
        assert!(verify_input_file("Cargo.toml").is_ok());
        assert!(verify_input_file("noexits").is_err());
    }
}
