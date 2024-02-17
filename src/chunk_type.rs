#![allow(unused_variables)]

use std::convert::TryFrom;
use std::{fmt::Display, str::FromStr};

#[derive(Debug, Clone, PartialEq, Eq)]
struct ChunkType {
    data: [u8; 4],
}
impl ChunkType {
    fn new(value: [u8; 4]) -> ChunkType {
        ChunkType { data: value }
    }
    fn bytes(&self) -> [u8; 4] {
        self.data
    }
    fn get_significant_bit(&self, idx: usize) -> bool {
        if idx > 3 {
            panic!("valid range: 0-3");
        }
        match self.data.get(idx) {
            None => panic!("BUG: this byte should not be empty"),
            Some(bit) => bit.is_ascii_uppercase(),
        }
    }
    fn is_valid(&self) -> bool {
        if self.is_reserved_bit_valid() {
            true
        } else {
            false
        }
    }
    fn is_critical(&self) -> bool {
        let bit5 = self.get_significant_bit(0);
        if bit5 {
            true
        } else {
            false
        }
    }

    fn is_public(&self) -> bool {
        let bit5 = self.get_significant_bit(1);
        if bit5 {
            true
        } else {
            false
        }
    }
    fn is_reserved_bit_valid(&self) -> bool {
        let bit5 = self.get_significant_bit(2);
        if bit5 {
            true
        } else {
            false
        }
    }

    fn is_safe_to_copy(&self) -> bool {
        let bit5 = self.get_significant_bit(3);
        if bit5 {
            false
        } else {
            true
        }
    }
}
impl TryFrom<[u8; 4]> for ChunkType {
    type Error = String;
    fn try_from(bytes: [u8; 4]) -> Result<Self, String> {
        let new = Self::new(bytes);
        if new.is_valid() {
            return Ok(new);
        } else {
            return Err(format!("Invalid byte representation!"));
        }
    }
}
impl FromStr for ChunkType {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, String> {
        let as_bytes = s.bytes().into_iter().collect::<Vec<u8>>();
        if as_bytes.len() != 4 {
            return Err(format!("Invalid length"));
        }
        let arr = [as_bytes[0], as_bytes[1], as_bytes[2], as_bytes[3]];
        Self::try_from(arr)
    }
}
impl Display for ChunkType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let formatted = self.data.map(|bit| format!("{}", bit));
        let joined = formatted.join(",");
        write!(f, "[{joined}]")
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;
    use std::str::FromStr;

    #[test]
    pub fn test_chunk_type_from_bytes() {
        let expected = [82, 117, 83, 116];
        let actual = ChunkType::try_from([82, 117, 83, 116]).unwrap();

        assert_eq!(expected, actual.bytes());
    }

    #[test]
    pub fn test_chunk_type_from_str() {
        let expected = ChunkType::try_from([82, 117, 83, 116]).unwrap();
        let actual = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    pub fn test_chunk_type_is_critical() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_not_critical() {
        let chunk = ChunkType::from_str("ruSt").unwrap();
        assert!(!chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_public() {
        let chunk = ChunkType::from_str("RUSt").unwrap();
        assert!(chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_not_public() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(!chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_invalid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_safe_to_copy() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_chunk_type_is_unsafe_to_copy() {
        let chunk = ChunkType::from_str("RuST").unwrap();
        assert!(!chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_valid_chunk_is_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_valid());
    }

    #[test]
    pub fn test_invalid_chunk_is_valid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_valid());

        let chunk = ChunkType::from_str("Ru1t");
        assert!(chunk.is_err());
    }

    #[test]
    pub fn test_chunk_type_string() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(&chunk.to_string(), "RuSt");
    }

    #[test]
    pub fn test_chunk_type_trait_impls() {
        let chunk_type_1: ChunkType = TryFrom::try_from([82, 117, 83, 116]).unwrap();
        let chunk_type_2: ChunkType = FromStr::from_str("RuSt").unwrap();
        let _chunk_string = format!("{}", chunk_type_1);
        let _are_chunks_equal = chunk_type_1 == chunk_type_2;
    }
}
