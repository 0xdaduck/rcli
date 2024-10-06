mod base64;
mod csv;
mod genpass;

use std::path::Path;

pub use self::{base64::*, csv::*, genpass::*};

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
}

fn verify_input_file(filename: &str) -> Result<String, &'static str> {
    // if input is "-" or file exists
    if filename == "-" || Path::new(filename).exists() {
        Ok(filename.into())
    } else {
        Err("File does not exist")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_verify_input_file() {
        assert!(verify_input_file("Cargo.toml").is_ok());
        assert!(verify_input_file("-").is_ok());
        // assert!(verify_input_file("test.txt").is_err());
    }
}
