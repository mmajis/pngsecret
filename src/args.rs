use std::path::PathBuf;
use clap::{Parser, Subcommand, Args};


#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct Cli {
    #[command(subcommand)]
    pub command: PngSecretArgs,
}

#[derive(Subcommand, Debug)]
pub enum PngSecretArgs {
    Encode(EncodeArgs),
    Decode(DecodeArgs),
    Remove(RemoveArgs),
    Print(PrintArgs),
}

#[derive(Args, Debug)]
pub struct EncodeArgs {
    pub file_path: PathBuf,
    pub chunk_type: String,
    pub message: String,
}

#[derive(Args, Debug)]
pub struct DecodeArgs {
    pub file_path: PathBuf,
    pub chunk_type: String,
}

#[derive(Args, Debug)]
pub struct RemoveArgs {
    pub file_path: PathBuf,
    pub chunk_type: String,
}

#[derive(Args, Debug)]
pub struct PrintArgs {
    pub file_path: PathBuf,
}