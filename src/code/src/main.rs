mod args;
mod chunk;
mod chunk_type;
mod commands;
pub mod png;

use crate::args::PngMeArgs;
use crate::commands::{decode, encode, print_chunks, remove};

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
    let args = todo!();

    match args {
        PngMeArgs::Encode(encode_args) => encode(encode_args),
        PngMeArgs::Decode(decode_args) => decode(decode_args),
        PngMeArgs::Remove(remove_args) => remove(remove_args),
        PngMeArgs::Print(print_args) => print_chunks(print_args),
    }
}
