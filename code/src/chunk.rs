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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::chunk_type::ChunkType;
    const CHUNK_MESSAGE: &str = "This is where your secret message will be!";
    const CHUNK_DATA_LENGTH: u32 = CHUNK_MESSAGE.len() as u32;
    const CHUNK_STR: &str = "rUSt";
    const CHUNK_CRC: u32 = 562223355;

    fn testing_chunk() -> Chunk {
        let chunk_type = CHUNK_STR.as_bytes();
        let message_bytes = CHUNK_MESSAGE.as_bytes();

        let chunk_data: Vec<u8> = CHUNK_DATA_LENGTH
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(CHUNK_CRC.to_be_bytes().iter())
            .copied()
            .collect();
        
        Chunk::try_from(chunk_data.as_ref()).unwrap()
    }

    #[test]
    fn test_new_chunk() {
        let chunk_type: ChunkType = CHUNK_STR.parse().unwrap();
        let data = CHUNK_MESSAGE.as_bytes().to_vec();
        let chunk = Chunk::new(chunk_type, data);
        assert_eq!(chunk.length(), CHUNK_DATA_LENGTH);
        assert_eq!(chunk.crc(), CHUNK_CRC);
    }

    #[test]
    fn test_chunk_length() {
        let chunk = testing_chunk();
        assert_eq!(chunk.length(), 42);
    }

    #[test]
    fn test_chunk_type() {
        let chunk = testing_chunk();
        assert_eq!(chunk.chunk_type().to_string(), String::from(CHUNK_STR));
    }

    #[test]
    fn test_chunk_string() {
        let chunk = testing_chunk();
        let chunk_string = chunk.data_as_string().unwrap();
        let expected_chunk_string = String::from(CHUNK_MESSAGE);
        assert_eq!(chunk_string, expected_chunk_string);
    }

    #[test]
    fn test_chunk_crc() {
        let chunk = testing_chunk();
        assert_eq!(chunk.crc(), CHUNK_CRC);
    }

    #[test]
    fn test_valid_chunk_from_bytes() {
        let chunk_type = CHUNK_STR.as_bytes();
        let message_bytes = CHUNK_MESSAGE.as_bytes();

        let chunk_data: Vec<u8> = CHUNK_DATA_LENGTH
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(CHUNK_CRC.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk = Chunk::try_from(chunk_data.as_ref()).unwrap();

        let chunk_string = chunk.data_as_string().unwrap();
        let expected_chunk_string = String::from(CHUNK_MESSAGE);

        assert_eq!(chunk.length(), 42);
        assert_eq!(chunk.chunk_type().to_string(), String::from(CHUNK_STR));
        assert_eq!(chunk_string, expected_chunk_string);
        assert_eq!(chunk.crc(), CHUNK_CRC);
    }

    #[test]
    fn test_invalid_chunk_from_bytes() {
        let chunk_type = CHUNK_STR.as_bytes();
        let message_bytes = CHUNK_MESSAGE.as_bytes();
        let crc: u32 = CHUNK_CRC - 1;

        let chunk_data: Vec<u8> = CHUNK_DATA_LENGTH
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

    #[test]
    pub fn test_chunk_trait_impls() {
        let chunk_type = CHUNK_STR.as_bytes();
        let message_bytes = CHUNK_MESSAGE.as_bytes();

        let chunk_data: Vec<u8> = CHUNK_DATA_LENGTH
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(CHUNK_CRC.to_be_bytes().iter())
            .copied()
            .collect();
        
        let chunk: Chunk = TryFrom::try_from(chunk_data.as_ref()).unwrap();
        
        let _chunk_string = format!("{}", chunk);
    }
}
