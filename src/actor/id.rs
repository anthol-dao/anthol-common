use candid::CandidType;
use ic_stable_structures::{storable::Bound, Storable};
use std::{
    borrow::Cow,
    fmt,
    hash::{Hash, Hasher},
};

const CAPITAL_MAP_SIZE: usize = 3;
const MAX_LENGTH_IN_BYTES: usize = 18;
const BYTE_LENGTH: usize = CAPITAL_MAP_SIZE + MAX_LENGTH_IN_BYTES;

/// Human readable id for actor.
///
/// ActorIds can contain alphabets, numbers, hyphens and underscores, and are case-insensitive but displayed in case-sensitive format.
/// Length of an ActorId must be between 3 and 24 characters.
#[derive(Debug, Clone, Copy, CandidType, Default)]
pub struct ActorId([u8; BYTE_LENGTH]); // 3 bytes for capital character map and 18 bytes for id

impl ActorId {
    /// The maximum length of an ActorId in characters
    pub const MAX_LENGTH: usize = 24;
    /// The minimum length of an ActorId in characters
    pub const MIN_LENGTH: usize = 3;
    /// The maximum length of an ActorId in bytes
    pub const MAX_LENGTH_IN_BYTES: usize = MAX_LENGTH_IN_BYTES;
    /// The minimum length of an ActorId in bytes
    pub const MIN_LENGTH_IN_BYTES: usize = 3;

    const CAPITAL_MAP_SIZE: usize = CAPITAL_MAP_SIZE;
    const MIN_LENGTH_IN_BYTES_WITH_CAPITAL_MAP: usize =
        Self::MIN_LENGTH_IN_BYTES + Self::CAPITAL_MAP_SIZE;
    const MAX_LENGTH_IN_BYTES_WITH_CAPITAL_MAP: usize =
        Self::MAX_LENGTH_IN_BYTES + Self::CAPITAL_MAP_SIZE;

    const BITS_PER_CHAR: usize = 6;
    const CHAR_MASK: u8 = 0b0011_1111; // 6 bits mask

    const ALPHABET_LITERAL_OFFSET: u8 = b'a' - 1; // 1-based index
    const CAPITAL_LITERAL_OFFSET: u8 = b'A' - 1; // 1-based index
    const NUMERIC_LITERAL_OFFSET: u8 = b'0' - 1 - 26; // 1-based index + 26 for alphabets

    /// Create a new ActorId from a string
    pub fn new<S: AsRef<str>>(id_str: S) -> Result<Self, ActorIdError> {
        Self::from_str_core(id_str.as_ref())
    }

    fn from_str_core(s: &str) -> Result<Self, ActorIdError> {
        let s = s.trim();
        let len = s.chars().count();
        if len < Self::MIN_LENGTH {
            return Err(ActorIdError::StringTooShort);
        } else if len > Self::MAX_LENGTH {
            return Err(ActorIdError::StringTooLong);
        }

        let mut bytes = [0; Self::MAX_LENGTH_IN_BYTES_WITH_CAPITAL_MAP];

        let mut bit_position = 0;

        for (position, c) in s.chars().enumerate() {
            let value = match c {
                'a'..='z' => c as u8 - Self::ALPHABET_LITERAL_OFFSET,
                'A'..='Z' => {
                    let map_byte_index = position / 8;
                    let map_bit_offset = position % 8;
                    unsafe {
                        *bytes.get_unchecked_mut(map_byte_index) |= 1 << map_bit_offset;
                    }

                    c as u8 - Self::CAPITAL_LITERAL_OFFSET
                }
                '0'..='9' => c as u8 - Self::NUMERIC_LITERAL_OFFSET,
                '-' => 37,
                '_' => 38,
                _ => return Err(ActorIdError::InvalidCharacter(c)),
            };

            let byte_index = bit_position / 8 + Self::CAPITAL_MAP_SIZE;
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
            len @ Self::MIN_LENGTH_IN_BYTES_WITH_CAPITAL_MAP
                ..=Self::MAX_LENGTH_IN_BYTES_WITH_CAPITAL_MAP => {
                let mut bytes = [0; Self::MAX_LENGTH_IN_BYTES_WITH_CAPITAL_MAP];
                unsafe {
                    std::ptr::copy_nonoverlapping(slice.as_ptr(), bytes.as_mut_ptr(), len);
                }
                Some(Self(bytes))
            }
            _ => None,
        }
    }

    /// Create a new ActorId from a byte slice
    pub fn from_slice(slice: &[u8]) -> Self {
        match Self::from_slice_core(slice) {
            Some(id) => id,
            None => panic!("slice length out of range"),
        }
    }

    /// Try to create a new ActorId from a byte slice
    pub fn try_from_slice(slice: &[u8]) -> Result<Self, ActorIdError> {
        match Self::from_slice_core(slice) {
            Some(id) => Ok(id),
            None => {
                let len = slice.len();
                if len < Self::MIN_LENGTH_IN_BYTES_WITH_CAPITAL_MAP {
                    Err(ActorIdError::BytesTooShort)
                } else {
                    Err(ActorIdError::BytesTooLong)
                }
            }
        }
    }

    /// Get the byte representation of the ActorId
    pub fn as_slice(&self) -> &[u8] {
        for i in
            Self::MIN_LENGTH_IN_BYTES_WITH_CAPITAL_MAP..Self::MAX_LENGTH_IN_BYTES_WITH_CAPITAL_MAP
        {
            if unsafe { *self.0.get_unchecked(i) } == 0 {
                return unsafe { self.0.get_unchecked(..i) };
            }
        }

        &self.0
    }
}

impl PartialEq for ActorId {
    fn eq(&self, other: &Self) -> bool {
        self.0[Self::CAPITAL_MAP_SIZE..] == other.0[Self::CAPITAL_MAP_SIZE..]
    }
}

impl Eq for ActorId {}

impl Hash for ActorId {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0[Self::CAPITAL_MAP_SIZE..].hash(state);
    }
}

impl PartialOrd for ActorId {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ActorId {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0[Self::CAPITAL_MAP_SIZE..].cmp(&other.0[Self::CAPITAL_MAP_SIZE..])
    }
}

impl fmt::Display for ActorId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut result = String::with_capacity(Self::MAX_LENGTH);
        let mut bit_position = 0;
        let mut position = 0;

        while position < Self::MAX_LENGTH {
            let byte_index = bit_position / 8 + Self::CAPITAL_MAP_SIZE;
            let bit_offset = bit_position % 8;

            let value = unsafe {
                if bit_offset <= 2 {
                    (*self.0.get_unchecked(byte_index) >> bit_offset) & Self::CHAR_MASK
                } else {
                    ((*self.0.get_unchecked(byte_index) >> bit_offset)
                        | (*self.0.get_unchecked(byte_index + 1) << (8 - bit_offset)))
                        & Self::CHAR_MASK
                }
            };

            let char = match value {
                0 => break,
                1..=26 => {
                    let map_byte_index = position / 8;
                    let map_bit_offset = position % 8;
                    let is_capital = unsafe {
                        (*self.0.get_unchecked(map_byte_index) >> map_bit_offset) & 1 == 1
                    };

                    if is_capital {
                        (value + Self::CAPITAL_LITERAL_OFFSET) as char
                    } else {
                        (value + Self::ALPHABET_LITERAL_OFFSET) as char
                    }
                }
                27..=36 => (value + Self::NUMERIC_LITERAL_OFFSET) as char,
                37 => '-',
                38 => '_',
                _ => unreachable!(),
            };

            result.push(char);
            bit_position += Self::BITS_PER_CHAR;
            position += 1;
        }

        f.write_str(&result)
    }
}

impl std::str::FromStr for ActorId {
    type Err = ActorIdError;

    /// Parse a string into an ActorId
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_str_core(s)
    }
}

impl TryFrom<&str> for ActorId {
    type Error = ActorIdError;

    /// Try to convert a string into an ActorId
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::from_str_core(value)
    }
}

impl TryFrom<&[u8]> for ActorId {
    type Error = ActorIdError;

    /// Try to convert a byte slice into an ActorId
    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        Self::try_from_slice(bytes)
    }
}

impl TryFrom<Vec<u8>> for ActorId {
    type Error = ActorIdError;

    /// Try to convert a byte vector into an ActorId
    fn try_from(bytes: Vec<u8>) -> Result<Self, Self::Error> {
        Self::try_from(bytes.as_slice())
    }
}

impl TryFrom<&Vec<u8>> for ActorId {
    type Error = ActorIdError;

    /// Try to convert a byte vector reference into an ActorId
    fn try_from(bytes: &Vec<u8>) -> Result<Self, Self::Error> {
        Self::try_from(bytes.as_slice())
    }
}

impl AsRef<[u8]> for ActorId {
    /// Get the byte representation of the ActorId
    fn as_ref(&self) -> &[u8] {
        self.as_slice()
    }
}

// Serialization
impl serde::Serialize for ActorId {
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
    use super::ActorId;
    use std::convert::TryFrom;

    // Simple visitor for deserialization from bytes. We don't support other number types
    // as there's no need for it.
    pub(super) struct ActorIdVisitor;

    impl<'de> serde::de::Visitor<'de> for ActorIdVisitor {
        type Value = super::ActorId;

        fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            formatter.write_str("bytes or string")
        }

        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            ActorId::from_str_core(v).map_err(E::custom)
        }

        fn visit_bytes<E>(self, value: &[u8]) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            ActorId::try_from(value).map_err(E::custom)
        }
    }
}

impl<'de> serde::Deserialize<'de> for ActorId {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<ActorId, D::Error> {
        use serde::de::Error;
        if deserializer.is_human_readable() {
            deserializer
                .deserialize_str(deserialize::ActorIdVisitor)
                .map_err(D::Error::custom)
        } else {
            deserializer
                .deserialize_bytes(deserialize::ActorIdVisitor)
                .map_err(D::Error::custom)
        }
    }
}

impl Storable for ActorId {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Borrowed(self.as_slice())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Self::from_slice(&bytes)
    }

    const BOUND: Bound = Bound::Bounded {
        max_size: 21,
        is_fixed_size: false,
    };
}

/// Errors that can occur when working with ActorIds
#[derive(thiserror::Error, Debug, Clone, PartialEq, Eq)]
pub enum ActorIdError {
    #[error("ActorId is longer than {} characters.", ActorId::MAX_LENGTH)]
    StringTooLong,

    #[error("ActorId is shorter than {} characters.", ActorId::MIN_LENGTH)]
    StringTooShort,

    #[error("Bytes is longer than {} bytes.", ActorId::MAX_LENGTH_IN_BYTES)]
    BytesTooLong,

    #[error("Bytes is shorter than {} bytes.", ActorId::MIN_LENGTH_IN_BYTES)]
    BytesTooShort,

    #[error("InvalActorId character '{0}' in ActorId.")]
    InvalidCharacter(char),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_actor_id_new() {
        let id = ActorId::new("test").unwrap();
        assert_eq!(id.to_string(), "test");

        let id = ActorId::new("Anthol_User").unwrap();
        assert_eq!(id.to_string(), "Anthol_User");

        let id = ActorId::new("Anthol_User-123").unwrap();
        assert_eq!(id.to_string(), "Anthol_User-123");

        let id = ActorId::new("AntholUser_").unwrap();
        assert_eq!(id.to_string(), "AntholUser_");

        let id = ActorId::new("z".repeat(24).as_str()).unwrap();
        assert_eq!(id.to_string(), "z".repeat(24));

        assert_eq!(ActorId::new("id"), Err(ActorIdError::StringTooShort));

        assert_eq!(
            ActorId::new("z".repeat(25).as_str()),
            Err(ActorIdError::StringTooLong)
        );

        assert_eq!(
            ActorId::new("id!"),
            Err(ActorIdError::InvalidCharacter('!'))
        );

        assert_eq!(
            ActorId::new("アイディー"),
            Err(ActorIdError::InvalidCharacter('ア'))
        );
    }

    #[test]
    fn test_id_from_str() {
        use std::str::FromStr;

        let id = ActorId::from_str("Anthol_User").unwrap();
        assert_eq!(id.to_string(), "Anthol_User");

        let id = ActorId::from_str("Anthol_User-123").unwrap();
        assert_eq!(id.to_string(), "Anthol_User-123");
    }

    #[test]
    fn test_id_try_from_slice() {
        let id = ActorId::new("Anthol_User").unwrap();
        let bytes = id.as_slice();
        let id2 = ActorId::try_from(bytes).unwrap();
        assert_eq!(id, id2);
        assert_eq!(id.to_string(), id2.to_string());
    }

    #[test]
    fn test_id_equality() {
        let id1 = ActorId::new("Anthol_User").unwrap();
        let id2 = ActorId::new("anthol_user").unwrap();
        assert_eq!(id1, id2);

        let id1 = ActorId::new("Anthol_User").unwrap();
        let id2 = ActorId::new("Anthol_User-123").unwrap();
        assert_ne!(id1, id2);
    }

    #[test]
    fn test_id_ordering() {
        let id1 = ActorId::new("Anthol_User").unwrap();
        let id2 = ActorId::new("anthol_user").unwrap();
        assert_eq!(id1, id2);

        let id1 = ActorId::new("Anthol_User").unwrap();
        let id2 = ActorId::new("Anthol_User-123").unwrap();
        assert!(id1 < id2);
    }

    #[test]
    fn test_id_serde() {
        let id = ActorId::new("Anthol_User").unwrap();
        let json = serde_json::to_string(&id).unwrap();
        assert_eq!(json, "\"Anthol_User\"");

        let id_json: ActorId = serde_json::from_str(&json).unwrap();
        assert_eq!(id, id_json);
        assert_eq!(id.to_string(), id_json.to_string());

        let bin = bincode::serialize(&id).unwrap();
        let id_bincode: ActorId = bincode::deserialize(&bin).unwrap();
        assert_eq!(id, id_bincode);
        assert_eq!(id.to_string(), id_bincode.to_string());
    }
}
