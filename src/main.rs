use clap::Parser;
use converet::{process_csv, Options, SubCommand};

fn main() -> anyhow::Result<()> {
    let options = Options::parse();
    match options.cmd {
        SubCommand::Csv(options) => process_csv(&options.input, &options.output)?,
    }
    Ok(())
}
