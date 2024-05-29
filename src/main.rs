use clap::Parser;
use cli_tools::{
    generate_password, process_csv, process_decode, process_encode, Base64Options, Options,
    SubCommand,
};

fn main() -> anyhow::Result<()> {
    let options = Options::parse();
    match options.cmd {
        SubCommand::Csv(options) => {
            let output = if let Some(output) = options.output {
                output.clone()
            } else {
                format!("output.{}", options.format)
            };
            process_csv(&options.input, output, options.format)?;
        }
        SubCommand::GeneratePassword(options) => {
            generate_password(
                options.length,
                options.uppercase,
                options.lowercase,
                options.digits,
                options.special,
            )?;
        }
        SubCommand::Base64(options) => match options {
            Base64Options::Encode(options) => {
                process_encode(&options.input, options.format)?;
            }
            Base64Options::Decode(options) => {
                process_decode(&options.input, options.format)?;
            }
        },
    }
    Ok(())
}
