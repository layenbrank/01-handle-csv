use clap::Parser;

#[derive(Debug, Parser)]
pub struct GeneratePasswordOptions {
    #[arg(short, long = "length", default_value_t = 16)]
    pub length: u8,
    #[arg(long = "uppercase", default_value_t = true)]
    pub uppercase: bool,
    #[arg(long = "lowercase", default_value_t = true)]
    pub lowercase: bool,
    #[arg(long = "digits", default_value_t = true)]
    pub digits: bool,
    #[arg(long = "special", default_value_t = true)]
    pub special: bool,
}
