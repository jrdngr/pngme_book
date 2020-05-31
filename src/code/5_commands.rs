use std::fs;
use std::str::FromStr;

use crate::setup::{Error, Result};
use crate::args::{DecodeArgs, EncodeArgs, PrintArgs, RemoveArgs};
use crate::png::{Chunk, ChunkType, Png};

pub fn encode(args: EncodeArgs) -> Result<()> {
    // Write me!
}

pub fn decode(args: DecodeArgs) -> Result<()> {
    // Write me!
}

pub fn remove(args: RemoveArgs) -> Result<()> {
    // Write me!
}

pub fn print_chunks(args: PrintArgs) -> Result<()> {
    // Write me!
}


#[cfg(test)]
mod assignment_tests {
    use super::*;
}
