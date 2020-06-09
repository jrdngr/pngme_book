use std::convert::TryFrom;
use std::fmt;
use std::fs;
use std::io::{BufReader, Read};
use std::path::Path;
use std::str::FromStr;

use crate::{Error, Result};
use crate::chunk::Chunk;
use crate::chunk_type::ChunkType;

/// A PNG container as described by the PNG spec
/// http://www.libpng.org/pub/png/spec/1.2/PNG-Contents.html
#[derive(Debug)]
pub struct Png {
    // Write me!
}

impl Png {
    // Fill in this array with the correct values per the PNG spec
    pub const STANDARD_HEADER: [u8; 8] = [0, 0, 0, 0, 0, 0, 0, 0];

    /// Creates a `Png` from a list of chunks using the correct header
    pub fn from_chunks(chunks: Vec<Chunk>) -> Self {
        todo!()
    }

    /// Creates a `Png` from a file path
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        todo!()
    }

    /// Appends a chunk to the end of this `Png` file's `Chunk` list.
    pub fn append_chunk(&mut self, chunk: Chunk) {
        todo!()
    }

    /// Searches for a `Chunk` with the specified `chunk_type` and removes the first
    /// matching `Chunk` from this `Png` list of chunks.
    pub fn remove_chunk(&mut self, chunk_type: &str) -> Result<Chunk> {
        todo!()
    }

    /// The header of this PNG.
    pub fn header(&self) -> &[u8; 8] {
        todo!()
    }

    /// Lists the `Chunk`s stored in this `Png`
    pub fn chunks(&self) -> &[Chunk] {
        todo!()
    }

    /// Searches for a `Chunk` with the specified `chunk_type` and returns the first
    /// matching `Chunk` from this `Png`.
    pub fn chunk_by_type(&self, chunk_type: &str) -> Option<&Chunk> {
        todo!()
    }

    /// Returns this `Png` as a byte sequence.
    /// These bytes will contain the header followed by the bytes of all of the chunks.
    pub fn as_bytes(&self) -> Vec<u8> {
        todo!()
    }
}

impl TryFrom<&[u8]> for Png {
    type Error = Error;

    fn try_from(bytes: &[u8]) -> Result<Png> {
        todo!()
    }
}

impl fmt::Display for Png {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}
