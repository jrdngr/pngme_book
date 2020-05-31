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
