use clap::Parser;
use std::path::Path;

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
}

#[derive(Debug, Parser)]
pub struct CsvOptions {
    #[arg(short, long = "input", value_parser=verify_input_file)]
    pub input: String,
    #[arg(short, long = "output", default_value = "output.json")]
    pub output: String,
    #[arg(short, long, default_value_t = ',')]
    pub delimiter: char,
    #[arg(long = "header", default_value_t = true)]
    pub header: bool,
}

fn verify_input_file(filename: &str) -> Result<String, String> {
    if Path::new(filename).exists() {
        Ok(filename.to_string())
    } else {
        Err(format!("File not found: {}", filename))
    }
}
