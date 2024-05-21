use clap::Parser;
use csvtools::{generate_password, process_csv, Options, SubCommand};

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
    }
    Ok(())
}
