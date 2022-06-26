use std::cmp::PartialEq;
use std::convert::TryFrom;
use std::fmt;
use std::result::Result;
use std::str::FromStr;
use std::string::ToString;
#[derive(Debug, Clone)]
pub struct ChunkType {
    datatype: [u8; 4],
}

impl TryFrom<[u8; 4]> for ChunkType {
    type Error = &'static str;
    fn try_from(value: [u8; 4]) -> Result<Self, Self::Error> {
        Ok(ChunkType { datatype: value })
    }
}

pub fn is_valid_byte(byte: u8) -> bool {
    (byte >= 65 && byte <= 90) || (byte >= 97 && byte <= 122)
}

impl ChunkType {
    pub fn bytes(&self) -> [u8; 4] {
        self.datatype
    }
    pub fn is_critical(&self) -> bool {
        (self.datatype[0] & (1 << 5)) == 0
    }
    pub fn is_public(&self) -> bool {
        (self.datatype[1] & (1 << 5)) == 0
    }

    pub fn is_reserved_bit_valid(&self) -> bool {
        (self.datatype[2] & (1 << 5)) == 0
    }

    pub fn is_safe_to_copy(&self) -> bool {
        (self.datatype[3] & (1 << 5)) == (1 << 5)
    }

    pub fn is_valid(&self) -> bool {
        if !self.is_reserved_bit_valid() {
            return false;
        }
        for i in 0..4 {
            if !is_valid_byte(self.datatype[i]) {
                return false;
            }
        }
        return true;
    }
    pub fn isAlpha(&self) -> bool {
        for i in 0..4 {
            if !is_valid_byte(self.datatype[i]) {
                return false;
            }
        }
        return true;
    }
}

impl PartialEq for ChunkType {
    fn eq(&self, other: &Self) -> bool {
        self.datatype == other.datatype
    }
}

impl FromStr for ChunkType {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 4 {
            return Err("The length of the string passed should be equal to 4");
        }
        let byte_arr = s.as_bytes();
        let mut new_arr: [u8; 4] = [0; 4];

        new_arr[..4].copy_from_slice(&byte_arr[..4]);
        let new_chunk = ChunkType { datatype: new_arr };
        if !new_chunk.isAlpha() {
            return Err(
                "The type codes are restricted to consist of uppercase and lowercase ASCII letters",
            );
        }

        Ok(ChunkType { datatype: new_arr })
    }
}

impl fmt::Display for ChunkType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", std::str::from_utf8(&self.datatype).unwrap())
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
