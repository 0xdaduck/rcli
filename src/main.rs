// rcli csv -i input.csv -o output.json --header -d ','

use clap::Parser;
use rcli::{process_csv, process_genpass, Opts, Subcommand};

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
            // let output = if let Some(output) = opts.output {
            //     output.clone()
            // } else {
            //     format!("password.{}", opts.format)
            // };
            // rcli::;
            let passwd = process_genpass(
                opts.length,
                opts.uppercase,
                opts.lowercase,
                opts.number,
                opts.symbol,
            )?;
            println!("{}", passwd);
        }
    }
    Ok(())
}
