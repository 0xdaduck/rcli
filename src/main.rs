// rcli csv -i input.csv -o output.json --header -d ','

use clap::Parser;
use rcli::{
    process_csv, process_decode, process_encode, process_genpass, Base64SubCommand, Opts,
    Subcommand,
};

fn main() -> anyhow::Result<()> {
    let opts: Opts = Opts::parse();

    match opts.cmd {
        Subcommand::Csv(opts) => {
            let output = if let Some(output) = opts.output {
                output.clone()
            } else {
                format!("output.{}", opts.format)
            };
            process_csv(&opts.input, output, opts.format)?;
        }
        Subcommand::GenPass(opts) => {
            process_genpass(
                opts.length,
                opts.uppercase,
                opts.lowercase,
                opts.number,
                opts.symbol,
            )?;
        }
        Subcommand::Base64(subcmd) => {
            let output = match subcmd {
                Base64SubCommand::Encode(opts) => process_encode(&opts.input, opts.format)?,
                Base64SubCommand::Decode(opts) => process_decode(&opts.input, opts.format)?,
            };
            println!("{}", output)
        }
    }

    Ok(())
}
