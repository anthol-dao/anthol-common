use crate::id::{Id, IdError};
use candid::CandidType;
use derive_more::{AsRef, Display, From};
use ic_stable_structures::{storable::Bound, Storable};
use serde::{Deserialize, Serialize};
use std::str::FromStr;

/// Wrapper for the item id.
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
pub struct ItemId(Id);

impl ItemId {
    pub fn new<T: AsRef<str>>(id: T) -> Result<Self, IdError> {
        Ok(ItemId(Id::new(id)?))
    }
}

impl FromStr for ItemId {
    type Err = IdError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(ItemId(Id::from_str(s)?))
    }
}

impl TryFrom<&str> for ItemId {
    type Error = IdError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(ItemId(Id::from_str(value)?))
    }
}

impl TryFrom<String> for ItemId {
    type Error = IdError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Ok(ItemId(Id::from_str(&value)?))
    }
}

impl Storable for ItemId {
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
