use std::convert::TryFrom;
use std::fmt;
use std::io::{BufReader, Read};

use crate::{Error, Result};
use crate::png::ChunkType;

/// A validated PNG chunk. See the PNG Spec for more details
/// http://www.libpng.org/pub/png/spec/1.2/PNG-Structure.html
#[derive(Debug, Clone)]
pub struct Chunk {
    // Write me!
}

impl Chunk {
    /// Creates a new chunk from a validated `ChunkType` and some data.
    /// The length and CRC will be computed automatically.
    pub fn new(chunk_type: ChunkType, data: Vec<u8>) -> Self {
        todo!()
    }

    /// The length of the data portion of this chunk.
    pub fn length(&self) -> u32 {
        todo!()
    }

    /// The `ChunkType` of this chunk
    pub fn chunk_type(&self) -> &ChunkType {
        todo!()
    }

    /// The raw data contained in this chunk in bytes
    pub fn data(&self) -> &[u8] {
        todo!()
    }

    /// The CRC of this chunk
    pub fn crc(&self) -> u32 {
        todo!()
    }

    /// Returns the data stored in this chunk as a `String`. This function will return an error
    /// if the stored data is not valid UTF-8.
    pub fn data_as_string(&self) -> Result<String> {
        todo!()
    }

    /// Returns this chunk as a byte sequences described by the PNG spec.
    /// The following data is included in this byte sequence in order:
    /// 1. Length of the data *(4 bytes)*
    /// 2. Chunk type *(4 bytes)*
    /// 3. The data itself *(`length` bytes)*
    /// 4. The CRC of the chunk type and data *(4 bytes)*
    pub fn as_bytes(&self) -> Vec<u8> {
        todo!()
    }
}

impl TryFrom<&[u8]> for Chunk {
    type Error = Error;

    fn try_from(bytes: &[u8]) -> Result<Self> {
        todo!()
    }
}

impl fmt::Display for Chunk {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Chunk {{",)?;
        writeln!(f, "  Length: {}", self.length())?;
        writeln!(f, "  Type: {}", self.chunk_type())?;
        writeln!(f, "  Data: {} bytes", self.data().len())?;
        writeln!(f, "  Crc: {}", self.crc())?;
        writeln!(f, "}}",)?;
        Ok(())
    }
}
