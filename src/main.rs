#![allow(dead_code)]
mod args;
mod chunk;
mod chunk_type;
mod commands;
mod png;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

use clap::Parser;
use crate::args::{Cli, PngSecretArgs};

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        PngSecretArgs::Encode(args) => commands::encode(args),
        PngSecretArgs::Decode(args) => commands::decode(args),
        PngSecretArgs::Remove(args) => commands::remove(args),
        PngSecretArgs::Print(args) => commands::print(args),
    }
}
