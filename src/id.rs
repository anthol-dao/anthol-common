use candid::CandidType;
use ic_stable_structures::{storable::Bound, Storable};
use std::{borrow::Cow, fmt};

const MAX_LENGTH_IN_BYTES: usize = 16;

/// Human readable ID for items, markets, etc.
///
/// IDs are case-insensitive and can contain alphabets, numbers and hyphens.
/// Length of an ID must be between 3 and 21 characters.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, CandidType, Default)]
pub struct Id([u8; MAX_LENGTH_IN_BYTES]);

impl Id {
    /// The maximum length of an ID in characters
    pub const MAX_LENGTH: usize = 21;
    /// The minimum length of an ID in characters
    pub const MIN_LENGTH: usize = 3;
    /// The maximum length of an ID in bytes
    pub const MAX_LENGTH_IN_BYTES: usize = MAX_LENGTH_IN_BYTES;
    /// The minimum length of an ID in bytes
    pub const MIN_LENGTH_IN_BYTES: usize = 2;

    const BITS_PER_CHAR: usize = 6;
    const CHAR_MASK: u8 = 0b0011_1111; // 6 bits mask
    const DATA_BITS_CAPACITY: usize = Self::MAX_LENGTH * Self::BITS_PER_CHAR;

    const ALPHABET_LITERAL_OFFSET: u8 = b'a' - 1; // 1-based index
    const NUMERIC_LITERAL_OFFSET: u8 = b'0' - 1 - 26; // 1-based index + 26 for alphabets

    /// Create a new ID from a string
    pub fn new<S: AsRef<str>>(id_str: S) -> Result<Self, IdError> {
        Self::from_str_core(id_str.as_ref())
    }

    fn from_str_core(s: &str) -> Result<Self, IdError> {
        let s = s.trim().to_ascii_lowercase();
        let s_len = s.chars().count();

        if s_len > Self::MAX_LENGTH {
            return Err(IdError::StringTooLong);
        } else if s_len < Self::MIN_LENGTH {
            return Err(IdError::StringTooShort);
        }
        if s.starts_with('-') || s.ends_with('-') {
            return Err(IdError::InvalidHyphenPosition);
        }

        let mut bytes = [0u8; Self::MAX_LENGTH_IN_BYTES];
        let mut bit_position = 0;

        for c in s.chars() {
            let value = match c {
                'a'..='z' => c as u8 - Self::ALPHABET_LITERAL_OFFSET,
                '0'..='9' => c as u8 - Self::NUMERIC_LITERAL_OFFSET,
                '-' => 37,
                _ => return Err(IdError::InvalidCharacter(c)),
            };

            let byte_index = bit_position / 8;
            let bit_offset = bit_position % 8;

            unsafe {
                *bytes.get_unchecked_mut(byte_index) |= value << bit_offset;
                if bit_offset > 2 {
                    *bytes.get_unchecked_mut(byte_index + 1) |= value >> (8 - bit_offset);
                }
            }

            bit_position += Self::BITS_PER_CHAR;
        }

        Ok(Self(bytes))
    }

    fn from_slice_core(slice: &[u8]) -> Option<Self> {
        match slice.len() {
            len @ Self::MIN_LENGTH_IN_BYTES..=Self::MAX_LENGTH_IN_BYTES => {
                let mut bytes = [0; Self::MAX_LENGTH_IN_BYTES];
                unsafe {
                    std::ptr::copy_nonoverlapping(slice.as_ptr(), bytes.as_mut_ptr(), len);
                }
                Some(Self(bytes))
            }
            _ => None,
        }
    }

    /// Create a new ID from a byte slice
    pub fn from_slice(slice: &[u8]) -> Self {
        match Self::from_slice_core(slice) {
            Some(id) => id,
            None => panic!("slice length out of range"),
        }
    }

    /// Try to create a new ID from a byte slice
    pub fn try_from_slice(slice: &[u8]) -> Result<Self, IdError> {
        match Self::from_slice_core(slice) {
            Some(id) => Ok(id),
            None => {
                let len = slice.len();
                if len < Self::MIN_LENGTH_IN_BYTES {
                    Err(IdError::BytesTooShort)
                } else {
                    Err(IdError::BytesTooLong)
                }
            }
        }
    }

    /// Get the byte representation of the ID
    pub fn as_slice(&self) -> &[u8] {
        unsafe {
            for i in Self::MIN_LENGTH_IN_BYTES..Self::MAX_LENGTH_IN_BYTES {
                if *self.0.get_unchecked(i) == 0 {
                    return &self.0[..i];
                }
            }
        }
        &self.0
    }
}

impl fmt::Display for Id {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut result = String::with_capacity(Self::MAX_LENGTH);
        let mut bit_position = 0;

        unsafe {
            while bit_position < Self::DATA_BITS_CAPACITY {
                let byte_index = bit_position / 8;
                let bit_offset = bit_position % 8;

                let value = if bit_offset <= 2 {
                    (*self.0.get_unchecked(byte_index) >> bit_offset) & Self::CHAR_MASK
                } else {
                    ((*self.0.get_unchecked(byte_index) >> bit_offset)
                        | (*self.0.get_unchecked(byte_index + 1) << (8 - bit_offset)))
                        & Self::CHAR_MASK
                };

                let char = match value {
                    0 => break,
                    1..=26 => (value + Self::ALPHABET_LITERAL_OFFSET) as char,
                    27..=36 => (value + Self::NUMERIC_LITERAL_OFFSET) as char,
                    37 => '-',
                    _ => unreachable!(),
                };

                result.push(char);
                bit_position += Self::BITS_PER_CHAR;
            }
        }

        f.write_str(&result)
    }
}

impl std::str::FromStr for Id {
    type Err = IdError;

    /// Parse a string into an ID
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_str_core(s)
    }
}

impl TryFrom<&str> for Id {
    type Error = IdError;

    /// Try to convert a string into an ID
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::from_str_core(value)
    }
}

impl TryFrom<&[u8]> for Id {
    type Error = IdError;

    /// Try to convert a byte slice into an ID
    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        Self::try_from_slice(bytes)
    }
}

impl TryFrom<Vec<u8>> for Id {
    type Error = IdError;

    /// Try to convert a byte vector into an ID
    fn try_from(bytes: Vec<u8>) -> Result<Self, Self::Error> {
        Self::try_from(bytes.as_slice())
    }
}

impl TryFrom<&Vec<u8>> for Id {
    type Error = IdError;

    /// Try to convert a byte vector reference into an ID
    fn try_from(bytes: &Vec<u8>) -> Result<Self, Self::Error> {
        Self::try_from(bytes.as_slice())
    }
}

impl AsRef<[u8]> for Id {
    /// Get the byte representation of the ID
    fn as_ref(&self) -> &[u8] {
        self.as_slice()
    }
}

// Serialization
impl serde::Serialize for Id {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        if serializer.is_human_readable() {
            self.to_string().serialize(serializer)
        } else {
            serializer.serialize_bytes(self.as_slice())
        }
    }
}

// Deserialization
mod deserialize {
    use super::Id;
    use std::convert::TryFrom;

    // Simple visitor for deserialization from bytes. We don't support other number types
    // as there's no need for it.
    pub(super) struct IdVisitor;

    impl<'de> serde::de::Visitor<'de> for IdVisitor {
        type Value = super::Id;

        fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            formatter.write_str("bytes or string")
        }

        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Id::from_str_core(v).map_err(E::custom)
        }

        fn visit_bytes<E>(self, value: &[u8]) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Id::try_from(value).map_err(E::custom)
        }
    }
}

impl<'de> serde::Deserialize<'de> for Id {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Id, D::Error> {
        use serde::de::Error;
        if deserializer.is_human_readable() {
            deserializer
                .deserialize_str(deserialize::IdVisitor)
                .map_err(D::Error::custom)
        } else {
            deserializer
                .deserialize_bytes(deserialize::IdVisitor)
                .map_err(D::Error::custom)
        }
    }
}

impl Storable for Id {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Borrowed(self.as_slice())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Self::from_slice(&bytes)
    }

    const BOUND: Bound = Bound::Bounded {
        max_size: Self::MAX_LENGTH_IN_BYTES as u32,
        is_fixed_size: false,
    };
}

/// Errors that can occur when working with IDs
#[derive(thiserror::Error, Debug, Clone, PartialEq, Eq)]
pub enum IdError {
    #[error("ID is longer than {} characters.", Id::MAX_LENGTH)]
    StringTooLong,

    #[error("ID is shorter than {} characters.", Id::MIN_LENGTH)]
    StringTooShort,

    #[error("Bytes is longer than {} bytes.", Id::MAX_LENGTH_IN_BYTES)]
    BytesTooLong,

    #[error("Bytes is shorter than {} bytes.", Id::MIN_LENGTH_IN_BYTES)]
    BytesTooShort,

    #[error("Invalid character '{0}' in ID.")]
    InvalidCharacter(char),

    #[error("ID cannot start or end with a hyphen.")]
    InvalidHyphenPosition,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_id_new() {
        let id = Id::new("abc").unwrap();
        assert_eq!(id.to_string(), "abc");

        let id = Id::new("abc-123").unwrap();
        assert_eq!(id.to_string(), "abc-123");

        let id = Id::new("wiggle-stool").unwrap();
        assert_eq!(id.to_string(), "wiggle-stool");

        let id = Id::new("z".repeat(21)).unwrap();
        assert_eq!(id.to_string(), "z".repeat(21));

        assert_eq!(Id::new("id"), Err(IdError::StringTooShort));
        assert_eq!(Id::new("z".repeat(22)), Err(IdError::StringTooLong));
        assert_eq!(Id::new("id-"), Err(IdError::InvalidHyphenPosition));
        assert_eq!(Id::new("id!"), Err(IdError::InvalidCharacter('!')));
    }

    #[test]
    fn test_id_bytes_size() {
        let id = Id::new("abc").unwrap();
        assert_eq!(id.as_slice().len(), 2);

        let id = Id::new("abc-123").unwrap();
        assert_eq!(id.as_slice().len(), 6);

        let id = Id::new("wiggle-stool").unwrap();
        assert_eq!(id.as_slice().len(), 9);

        let id = Id::new("z".repeat(21)).unwrap();
        assert_eq!(id.as_slice().len(), 16);
    }
}
