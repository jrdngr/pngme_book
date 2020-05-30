use std::convert::TryFrom;
use std::fmt;
use std::io::{BufReader, Read};
use std::str::FromStr;

use crate::png::ChunkType;

#[derive(Debug, Clone)]
pub struct Chunk {
    chunk_type: ChunkType,
    // Write me!
}

impl Chunk {
    pub fn from_bytes(data_length: u32, bytes: &[u8]) -> anyhow::Result<Self> {
        // Write me! 
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        // Write me!
    }

    // Write the rest of my methods!
}

impl fmt::Display for Chunk {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Chunk {{",)?;
        writeln!(f, "  Type: {}", self.chunk_type)?;
        // Write me!
        writeln!(f, "}}",)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
