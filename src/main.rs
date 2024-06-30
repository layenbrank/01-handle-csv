// cargo run --  csv --input test.json --output output.json --header --delimiter ";"

use clap::Parser;
use clitoos::{process_csv, Opts, Subcommand};

fn main() -> anyhow::Result<()> {
    let opts = Opts::parse();
    match opts.cmd {
        Subcommand::Csv(opts) => process_csv(&opts.input, &opts.output)?,
    }
    Ok(())
}
