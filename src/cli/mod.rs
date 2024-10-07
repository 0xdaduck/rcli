mod base64;
mod csv;
mod genpass;
mod http_cli;
mod text;

pub use self::{base64::*, csv::*, genpass::*, http_cli::*, text::*};
use crate::utils::{verify_file, verify_path};
use clap::Parser;

#[derive(Debug, Parser)]
#[command(name = "rcli", version, author,about="rust cli",long_about=None)]
pub struct Opts {
    #[command(subcommand)]
    pub cmd: Subcommand,
}

#[derive(Debug, Parser)]
pub enum Subcommand {
    #[command(name = "csv", about = "Show or Convert csv to other formats")]
    Csv(CsvOpts),

    #[command(name = "genpass", about = "Generate password")]
    GenPass(GenPassOpts),

    #[command(subcommand)]
    Base64(Base64SubCommand),

    #[command(subcommand)]
    Text(TextSubCommand),

    #[command(subcommand)]
    Http(HttpSubCommand),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_verify_input_file() {
        assert!(verify_file("Cargo.toml").is_ok());
        assert!(verify_file("-").is_ok());
    }
}
