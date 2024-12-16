use crate::id::{Id, IdError};
use candid::{CandidType, Principal};
use derive_more::{AsRef, Display, From, Into};
use ic_stable_structures::{storable::Bound, Storable};
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(
    CandidType,
    Clone,
    Copy,
    Serialize,
    Deserialize,
    Debug,
    Hash,
    Eq,
    PartialEq,
    PartialOrd,
    Ord,
    AsRef,
    From,
    Display,
    Into,
)]
pub struct StorePrincipal(Principal);

#[derive(
    CandidType,
    Clone,
    Copy,
    Serialize,
    Deserialize,
    Debug,
    Hash,
    Eq,
    PartialEq,
    PartialOrd,
    Ord,
    Default,
    AsRef,
    From,
    Display,
)]
pub struct StoreId(Id);

impl StoreId {
    pub fn new<T: AsRef<str>>(id: T) -> Result<Self, IdError> {
        Ok(StoreId(Id::new(id)?))
    }
}

impl FromStr for StoreId {
    type Err = IdError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(StoreId(Id::from_str(s)?))
    }
}

impl TryFrom<&str> for StoreId {
    type Error = IdError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(StoreId(Id::from_str(value)?))
    }
}

impl TryFrom<String> for StoreId {
    type Error = IdError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Ok(StoreId(Id::from_str(&value)?))
    }
}

impl Storable for StorePrincipal {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        self.0.to_bytes()
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Self(Principal::from_bytes(bytes))
    }

    const BOUND: Bound = Bound::Bounded {
        max_size: Principal::MAX_LENGTH_IN_BYTES as u32,
        is_fixed_size: false,
    };
}

impl Storable for StoreId {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        self.0.to_bytes()
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Self(Id::from_bytes(bytes))
    }

    const BOUND: Bound = Bound::Bounded {
        max_size: 16,
        is_fixed_size: false,
    };
}

/// Store name up to 50 characters
#[derive(
    CandidType,
    Clone,
    Serialize,
    Deserialize,
    Debug,
    Hash,
    Eq,
    PartialEq,
    PartialOrd,
    Ord,
    Default,
    AsRef,
    Display,
    Into,
)]
pub struct StoreName(String);

impl StoreName {
    pub fn new<T: AsRef<str>>(name: T) -> Result<Self, StoreNameError> {
        Self::from_str_core(name.as_ref())
    }

    fn from_str_core(s: &str) -> Result<Self, StoreNameError> {
        if s.chars().count() > 50 {
            return Err(StoreNameError::TooLong);
        }
        Ok(StoreName(s.to_string()))
    }
}

impl FromStr for StoreName {
    type Err = StoreNameError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_str_core(s)
    }
}

impl TryFrom<&str> for StoreName {
    type Error = StoreNameError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::from_str_core(value)
    }
}

impl TryFrom<String> for StoreName {
    type Error = StoreNameError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::from_str_core(&value)
    }
}

impl Storable for StoreName {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        self.0.to_bytes()
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Self(String::from_bytes(bytes))
    }

    const BOUND: Bound = Bound::Bounded {
        max_size: 200,
        is_fixed_size: false,
    };
}

#[derive(thiserror::Error, Debug, Clone, Serialize, Deserialize, CandidType)]
pub enum StoreNameError {
    #[error("Provided store name is too long")]
    TooLong,
}

#[derive(Debug, Clone, Serialize, Deserialize, CandidType)]
pub struct StoreInitArg {
    pub id: StoreId,
    pub name: StoreName,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_store_id() {
        let id = StoreId::new("test").unwrap();
        assert_eq!(id.to_string(), "test");
        assert_eq!(StoreId::from_str("test").unwrap(), id);
        assert_eq!(StoreId::try_from("test").unwrap(), id);
        assert_eq!(StoreId::try_from("test".to_string()).unwrap(), id);
    }

    #[test]
    fn test_store_name() {
        let name = StoreName::new("test").unwrap();
        assert_eq!(name.to_string(), "test");
        assert_eq!(StoreName::from_str("test").unwrap(), name);
        assert_eq!(StoreName::try_from("test").unwrap(), name);
        assert_eq!(StoreName::try_from("test".to_string()).unwrap(), name);

        let name = StoreName::new("𐍈".repeat(50)).unwrap(); // '𐍈' is 4 bytes character
        assert_eq!(name.to_string(), "𐍈".repeat(50));
        assert_eq!(name.to_bytes().len(), 200);
    }

    #[test]
    fn test_store_id_bytes_length() {
        let id = StoreId::new("abc").unwrap();
        let bytes = id.to_bytes();
        assert_eq!(bytes.len(), 2);
    }
}
