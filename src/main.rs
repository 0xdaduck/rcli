use std::fs;

use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
use clap::Parser;
use rcli::{
    get_content, get_reader, process_csv, process_decode, process_encode, process_genpass,
    process_text_genkey, process_text_sign, process_text_verify, Base64SubCommand, Opts,
    Subcommand, TextSubCommand,
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
            let password = process_genpass(
                opts.length,
                opts.uppercase,
                opts.lowercase,
                opts.number,
                opts.symbol,
            )?;
            println!("generate password:{}", password)
        }
        Subcommand::Base64(subcmd) => {
            let output = match subcmd {
                Base64SubCommand::Encode(opts) => process_encode(&opts.input, opts.format)?,
                Base64SubCommand::Decode(opts) => process_decode(&opts.input, opts.format)?,
            };
            println!("{}", output)
        }
        Subcommand::Text(subcmd) => match subcmd {
            TextSubCommand::Sign(opts) => {
                let mut reader = get_reader(&opts.input)?;
                let key = get_content(&opts.key)?;
                let sig = process_text_sign(&mut reader, &key, opts.format)?;
                let encode = URL_SAFE_NO_PAD.encode(sig);
                println!("{}", encode)
            }
            TextSubCommand::Verify(opts) => {
                let mut reader = get_reader(&opts.input)?;
                let key = get_content(&opts.key)?;
                let decoded = URL_SAFE_NO_PAD.decode(&opts.sig)?;
                let verify = process_text_verify(&mut reader, &key, &decoded, opts.format)?;
                if verify {
                    println!("Signature verified")
                } else {
                    println!("Signature not verified")
                }
            }
            TextSubCommand::GenerateKey(opts) => {
                let map = process_text_genkey(opts.format)?;
                for (k, v) in map {
                    fs::write(opts.output.join(k), v)?;
                }
            }
        },
    }

    Ok(())
}
