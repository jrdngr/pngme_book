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

    /// Since the purpose of this program is to encode messages, it can be useful to 
    /// create new chunks from a pair of strings representing the chunk type and message.
    /// 
    /// # Example
    ///
    /// ```
    /// # fn main() -> Result<(), Box<dyn std::error::Error> {
    /// let chunk = Chunk::from_strings("RuSt", "This is a secret message!")?;
    /// 
    /// assert_eq!(&chunk.chunk_type().to_string(), "RuSt");
    /// assert_eq!(&chunk.data_as_string()?, "This is a secret message!");
    /// # }
    /// ```
    pub fn from_strings(chunk_type: &str, data: &str) -> Result<Self> {
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

    /// The CRC of this chunk. If this chunk was created with `new` or `from_strings`, 
    /// the CRC is computed based on the data. If this chunk was created with `TryFrom<&[u8]>`,
    /// the CRC was read from the bytes per the PNG spec and validated against the data
    /// stored in this chunk.
    pub fn crc(&self) -> u32 {
        todo!()
    }

    /// Calculates a new CRC based on the data stored in this chunk. Returns true if the calculated
    /// CRC matches the stored CRC.
    pub fn is_crc_valid(&self) -> bool {
        todo!()
    }

    /// Returns the data stored in this chunk as a `String`. This function will return an error
    /// if the stored data is not valid UTF-8.
    pub fn data_as_string(&self) -> Result<String> {
        todo!()
    }

    /// Returns this chunk as a byte sequences described by the PNG spec.
    /// The following data is included in this byte sequence
    /// 1. Length of the data *(4 bytes)*
    /// 2. Chunk type *(4 bytes)*
    /// 3. The data itself *(`length` bytes)*
    /// 4. The CRC of the chunk type and data *(4 bytes)*
    pub fn as_bytes(&self) -> Vec<u8> {
        todo!()
    }

    /// Calculates the CRC of a `ChunkType` followed by some data
    pub fn calculate_crc(chunk_type: &ChunkType, data: &[u8]) -> u32 {
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    fn testing_chunk() -> Chunk {
        let chunk_type = ChunkType::from_str("RuSt").unwrap();
        let data: Vec<u8> = "This is where your secret message will be!"
            .bytes()
            .collect();
        Chunk::new(chunk_type, data)
    }

    #[test]
    fn test_chunk_length() {
        let chunk = testing_chunk();
        assert_eq!(chunk.length(), 42);
    }

    #[test]
    fn test_chunk_type() {
        let chunk = testing_chunk();
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
    }

    #[test]
    fn test_chunk_string() {
        let chunk = testing_chunk();
        let chunk_string = chunk.data_as_string().unwrap();
        let expected_chunk_string = String::from("This is where your secret message will be!");
        assert_eq!(chunk_string, expected_chunk_string);
    }

    #[test]
    fn test_chunk_crc() {
        let chunk = testing_chunk();
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_valid_chunk_from_bytes() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk = Chunk::try_from(chunk_data.as_ref()).unwrap();

        let chunk_string = chunk.data_as_string().unwrap();
        let expected_chunk_string = String::from("This is where your secret message will be!");

        assert_eq!(chunk.length(), 42);
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
        assert_eq!(chunk_string, expected_chunk_string);
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_invalid_chunk_from_bytes() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656333;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk = Chunk::try_from(chunk_data.as_ref());

        assert!(chunk.is_err());
    }
}
