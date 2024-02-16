#![allow(unused_variables)]

use crate::{Error, Result};
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
    fn get_significant_bit(&self, idx: usize) -> &u8 {
        if idx > 3 {
            panic!("valid range: 0-3");
        }
        match self.data.get(idx) {
            None => panic!("BUG: this byte should not be empty"),
            Some(bit) => {
                let mut current_bit = bit;
                for _ in 0..5 {
                    current_bit = &(current_bit % 2);
                }
                current_bit
            }
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
        if *bit5 == 0 {
            true
        } else {
            false
        }
    }

    fn is_public(&self) -> bool {
        let bit5 = self.get_significant_bit(1);
        if *bit5 == 0 {
            true
        } else {
            false
        }
    }
    fn is_reserved_bit_valid(&self) -> bool {
        let bit5 = self.get_significant_bit(2);
        if *bit5 == 0 {
            true
        } else {
            false
        }
    }

    fn is_safe_to_copy(&self) -> bool {
        let bit5 = self.get_significant_bit(3);
        if *bit5 == 1 {
            true
        } else {
            false
        }
    }
}
impl TryFrom<[u8; 4]> for ChunkType {
    type Error = Error;
    fn try_from(bytes: [u8; 4]) -> Result<Self> {
        let new = Self::new(bytes);
        if new.is_valid() {
            return Ok(new);
        } else {
            return Err(Error);
        }
    }
}
impl FromStr for ChunkType {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {}
}
impl Display for ChunkType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {}
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
