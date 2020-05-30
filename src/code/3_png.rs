use crate::Chunk;
use crate::ChunkType;

#[derive(Debug)]
pub struct Png {
    // Write me!
}

impl Png {
    pub const EXPECTED_HEADER: [u8; 8] = [ /* Fill me in */ ];

    // Write me!
}

#[cfg(test)]
mod assignment_tests {
    use super::*;

    fn testing_chunks() -> Vec<Chunk> {
        let mut chunks = Vec::new();

        chunks.push(Chunk::from_strings("FrSt", "I am the first chunk").unwrap());
        chunks.push(Chunk::from_strings("miDl", "I am another chunk").unwrap());
        chunks.push(Chunk::from_strings("LASt", "I am the last chunk").unwrap());

        chunks
    }

    #[test]
    fn test_valid_from_bytes() {
        let chunk_bytes: Vec<u8> = testing_chunks()
            .into_iter()
            .flat_map(|chunk| chunk.as_bytes())
            .collect();

        let bytes: Vec<u8> = Png::EXPECTED_HEADER
            .iter()
            .chain(chunk_bytes.iter())
            .copied()
            .collect();

        let png = Png::try_from(bytes.as_ref());
        
        assert!(png.is_ok());
    }

    #[test]
    fn test_invalid_header() {
        let chunk_bytes: Vec<u8> = testing_chunks()
            .into_iter()
            .flat_map(|chunk| chunk.as_bytes())
            .collect();

        let bytes: Vec<u8> =[13, 80, 78, 71, 13, 10, 26, 10]
            .iter()
            .chain(chunk_bytes.iter())
            .copied()
            .collect();

        let png = Png::try_from(bytes.as_ref());
        
        assert!(png.is_err());
    }

    #[test]
    fn test_invalid_chunk() {
        let mut chunk_bytes: Vec<u8> = testing_chunks()
            .into_iter()
            .flat_map(|chunk| chunk.as_bytes())
            .collect();

        #[rustfmt::skip]
        let mut bad_chunk = vec![
            0, 0, 0, 5,         // length
            32, 117, 83, 116,   // Chunk Type (bad)
            65, 64, 65, 66, 67, // Data
            1, 2, 3, 4, 5       // CRC (bad)
        ];

        chunk_bytes.append(&mut bad_chunk);
        
        let png = Png::try_from(chunk_bytes.as_ref());
        
        assert!(png.is_err());
    }
}
