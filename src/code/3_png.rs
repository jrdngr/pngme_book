pub mod chunk;

use std::fmt;
use std::fs;
use std::io::{BufReader, Read};
use std::path::Path;
use std::str::FromStr;

pub use chunk::{Chunk, ChunkType};

#[derive(Debug)]
pub struct Png {
    chunks: Vec<Chunk>,
    // Write me!
}

impl Png {
    pub fn from_bytes(bytes: &[u8]) -> anyhow::Result<Self> {
        // Write me!
    }

    pub fn from_file<P: AsRef<Path>>(path: P) -> anyhow::Result<Self> {
        // TODO: Hide me
    }

    pub fn insert_chunk(&mut self, chunk: Chunk) {
        // Write me!
    }

    pub fn remove_chunk(&mut self, chunk_type: &str) -> anyhow::Result<Chunk> {
        // Write me!
    }

    pub fn chunks(&self) -> &[Chunk] {
       // Write me!
    }

    pub fn chunk_by_type(&self, chunk_type: &str) -> Option<&Chunk> {
        // Write me!
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        // Write me!
    }
}

#[cfg(test)]
mod assignment_tests {
    use super::*;
}
