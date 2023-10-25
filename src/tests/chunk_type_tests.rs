#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_chunk_type_from_bytes() {
        let expected = [114, 85, 83, 116];
        let actual = ChunkType::try_from(expected).unwrap();

        assert_eq!(expected, actual.bytes());
    }

    #[test]
    pub fn test_chunk_type_from_str() {
        let expected = ChunkType::try_from([114, 85, 83, 116]).unwrap();
        let actual: ChunkType = "rUSt".parse().unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    pub fn test_chunk_type_is_critical() {
        let chunk: ChunkType = "RUSt".parse().unwrap();
        assert!(chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_not_critical() {
        let chunk: ChunkType = "ruSt".parse().unwrap();
        assert!(!chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_public() {
        let chunk: ChunkType = "rUSt".parse().unwrap();
        assert!(chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_not_public() {
        let chunk: ChunkType = "ruSt".parse().unwrap();
        assert!(!chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_valid() {
        let chunk: ChunkType = "rUSt".parse().unwrap();
        assert!(chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_invalid() {
        let chunk: ChunkType = "Rust".parse().unwrap();
        assert!(!chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_safe_to_copy() {
        let chunk: ChunkType = "rUSt".parse().unwrap();
        assert!(chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_chunk_type_is_unsafe_to_copy() {
        let chunk: ChunkType = "RuST".parse().unwrap();
        assert!(!chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_valid_chunk_is_valid() {
        let chunk: ChunkType = "rUSt".parse().unwrap();
        assert!(chunk.is_valid());

        let chunk: ChunkType = "RuST".parse().unwrap();
        assert!(chunk.is_valid());
    }

    #[test]
    pub fn test_invalid_character() {
        let chunk: Result<ChunkType> = "r1St".parse();
        assert!(chunk.is_err());
    }

    #[test]
    pub fn test_invalid_reserved() {
        let chunk: ChunkType = "rUst".parse().unwrap();
        assert!(!chunk.is_valid());
    }

    #[test]
    pub fn test_invalid_safetocopy() {
        let chunk: ChunkType = "RuSt".parse().unwrap();
        assert!(!chunk.is_valid());
    }

    #[test]
    pub fn test_chunk_type_string() {
        let chunk: ChunkType = "rUSt".parse().unwrap();
        assert_eq!(&chunk.to_string(), "rUSt");
    }

    #[test]
    pub fn test_chunk_type_trait_impls() {
        let chunk_type_1: ChunkType = [82, 117, 83, 116].try_into().unwrap();
        let chunk_type_2: ChunkType = "rUSt".parse().unwrap();
        let _chunk_string = format!("{}", chunk_type_1);
        let _are_chunks_equal = chunk_type_1 == chunk_type_2;
    }
}
