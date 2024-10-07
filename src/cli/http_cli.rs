use std::path::PathBuf;

use clap::Parser;

use super::verify_path;

#[derive(Debug, Parser)]
pub enum HttpSubCommand {
    #[command(name = "serve", about = "Serve a local HTTP server")]
    Serve(HttpServeOpts),
}

#[derive(Debug, Parser)]
pub struct HttpServeOpts {
    #[arg(long, value_parser=verify_path, default_value=".", help = "Path to serve")]
    pub path: PathBuf,
    #[arg(long, default_value_t = 8080u16, help = "Port to serve")]
    pub port: u16,
}
