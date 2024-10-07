use std::fs;

use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
use clap::Parser;
use rcli::{
    get_content, get_reader, process_csv, process_decode, process_encode, process_genpass,
    process_http_serve, process_text_genkey, process_text_sign, process_text_verify,
    Base64SubCommand, HttpSubCommand, Opts, Subcommand, TextSubCommand,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

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
            match opts.output {
                Some(output) => fs::write(output.join("password.txt"), password)?,
                None => println!("{}", password),
            }
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
        Subcommand::Http(subcmd) => match subcmd {
            HttpSubCommand::Serve(opts) => {
                process_http_serve(opts.path, opts.port).await?;
            }
        },
    }

    Ok(())
}
