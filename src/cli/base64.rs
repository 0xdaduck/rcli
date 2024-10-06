use core::fmt;
use std::str::FromStr;

use clap::Parser;

use super::verify_file;

#[derive(Debug, Parser)]
pub enum Base64SubCommand {
    #[command(name = "encode", about = "Encode string to base64")]
    Encode(Base64EncodeOpts),

    #[command(name = "decode", about = "Decode base64 string")]
    Decode(Base64DecodeOpts),
}

#[derive(Debug, Parser)]
pub struct Base64EncodeOpts {
    #[arg(short, long, value_parser=verify_file, default_value="-", help = "String to encode")]
    pub input: String,
    #[arg(long, default_value = "standard", value_parser=parse_base64_format, help = "Base64 format")]
    pub format: Base64Format,
}

#[derive(Debug, Parser)]
pub struct Base64DecodeOpts {
    #[arg(short, long, value_parser=verify_file, default_value="-", help = "String to decode")]
    pub input: String,
    #[arg(long, default_value = "standard", value_parser=parse_base64_format, help = "Base64 format")]
    pub format: Base64Format,
}

#[derive(Debug, Clone, Copy)]
pub enum Base64Format {
    Standard,
    UrlSafe,
}

fn parse_base64_format(format: &str) -> Result<Base64Format, anyhow::Error> {
    format.parse()
}

impl FromStr for Base64Format {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "standard" => Ok(Base64Format::Standard),
            "urlsafe" => Ok(Base64Format::UrlSafe),
            _ => Err(anyhow::anyhow!("Invalid base64 format")),
        }
    }
}

impl fmt::Display for Base64Format {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Base64Format::Standard => write!(f, "standard"),
            Base64Format::UrlSafe => write!(f, "urlsafe"),
        }
    }
}
