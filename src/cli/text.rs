use std::{fmt, path::PathBuf, str::FromStr};

use clap::Parser;

use crate::verify_path;

use super::verify_file;

#[derive(Debug, Parser)]
pub enum TextSubCommand {
    #[command(about = "Sign a message with your private/shared key")]
    Sign(TextSginOpts),

    #[command(about = "Verify a signed message")]
    Verify(TextVerifyOpts),

    #[command(about = "Generate a new private key")]
    GenerateKey(TextKeyGenerateOpts),
}

#[derive(Debug, Parser)]
pub struct TextKeyGenerateOpts {
    #[arg(short, long, default_value = "blake3", value_parser=parse_format)]
    pub format: TextSignFormat,
    #[arg(short, long, value_parser=verify_path)]
    pub output: PathBuf,
}

#[derive(Debug, Parser)]
pub struct TextSginOpts {
    #[arg(short, long, value_parser=verify_file, default_value="-")]
    pub input: String,
    #[arg(short, long, value_parser=verify_file)]
    pub key: String,
    #[arg(long, default_value = "blake3", value_parser=parse_format)]
    pub format: TextSignFormat,
}

#[derive(Debug, Parser)]
pub struct TextVerifyOpts {
    #[arg(short, long, value_parser=verify_file, default_value="-")]
    pub input: String,
    #[arg(short, long, value_parser=verify_file)]
    pub key: String,
    #[arg(short, long)]
    pub sig: String,
    #[arg(long, default_value = "blake3", value_parser=parse_format)]
    pub format: TextSignFormat,
}

#[derive(Debug, Clone, Copy)]
pub enum TextSignFormat {
    Blake3,
    Ed25519,
}

fn parse_format(format: &str) -> Result<TextSignFormat, anyhow::Error> {
    format.parse()
}

impl TextSignFormat {
    pub fn as_str(&self) -> &str {
        match self {
            TextSignFormat::Blake3 => "blake3",
            TextSignFormat::Ed25519 => "ed25519",
        }
    }
}

impl FromStr for TextSignFormat {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "blake3" => Ok(TextSignFormat::Blake3),
            "ed25519" => Ok(TextSignFormat::Ed25519),
            _ => Err(anyhow::anyhow!("Unknown format: {}", s)),
        }
    }
}

impl From<TextSignFormat> for &'static str {
    fn from(format: TextSignFormat) -> Self {
        match format {
            TextSignFormat::Blake3 => "blake3",
            TextSignFormat::Ed25519 => "ed25519",
        }
    }
}

impl fmt::Display for TextSignFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
