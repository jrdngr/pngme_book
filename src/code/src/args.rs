use std::path::PathBuf;

use clap::Clap;

#[derive(Clap, Debug)]
pub enum PngMeArgs {
    Encode(EncodeArgs),
    Decode(DecodeArgs),
    Remove(RemoveArgs),
    Print(PrintArgs),
}

#[derive(Clap, Debug)]
pub struct EncodeArgs {
    pub file: PathBuf,
    pub chunk: String,
    pub message: String,
    pub out: Option<PathBuf>,
}

#[derive(Clap, Debug)]
pub struct DecodeArgs {
    pub file: PathBuf,
    pub chunk: String,
}

#[derive(Clap, Debug)]
pub struct RemoveArgs {
    pub file: PathBuf,
    pub chunk: String,
}

#[derive(Clap, Debug)]
pub struct PrintArgs {
    pub file: PathBuf,
}
