use clap::Parser;
use csvtools::{process_csv, Options, SubCommand};

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
    }
    Ok(())
}
