use std::fs;
use std::str::FromStr;

use crate::args::{DecodeArgs, EncodeArgs, PrintArgs, RemoveArgs};
use crate::png::{Chunk, ChunkType, Png};

pub fn encode(args: EncodeArgs) -> anyhow::Result<()> {
    // Write me!
}

pub fn decode(args: DecodeArgs) -> anyhow::Result<()> {
    // Write me!
}

pub fn remove(args: RemoveArgs) -> anyhow::Result<()> {
    // Write me!
}

pub fn print_chunks(args: PrintArgs) -> anyhow::Result<()> {
    // Write me!
}


#[cfg(test)]
mod assignment_tests {
    use super::*;
}
