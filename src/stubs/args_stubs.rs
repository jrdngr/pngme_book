use std::path::PathBuf;

pub enum PngMeArgs {
    Encode(EncodeArgs),
    Decode(DecodeArgs),
    Remove(RemoveArgs),
    Print(PrintArgs),
}

pub struct EncodeArgs {
    // Write me!
}

pub struct DecodeArgs {
    // Write me!
}

pub struct RemoveArgs {
    // Write me!
}

pub struct PrintArgs {
    // Write me!
}


// If you use this structure for your args, you can also use the following code in your main.rs

mod args;
mod chunk;
mod chunk_type;
mod commands;
mod png;

use crate::args::PngMeArgs;
use crate::commands::{decode, encode, print_chunks, remove};

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
    todo!()
}
