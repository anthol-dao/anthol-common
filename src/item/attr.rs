use std::{fmt::Display, str::FromStr};

use anyhow::Error;
use candid::{CandidType, Decode, Deserialize, Encode};
use ic_stable_structures::{storable::Bound, Storable};
use serde::Serialize;
use std::borrow::Cow;

pub type AttrKey = u8;

/// The collection of keys for the attributes.
/// This is used to identify the attributes of an item.
#[derive(
    Hash,
    Eq,
    PartialEq,
    PartialOrd,
    Ord,
    Debug,
    CandidType,
    Serialize,
    Deserialize,
    Clone,
    Copy,
    Default,
)]
pub struct AttrKeys(pub [AttrKey; 4]);

impl Storable for AttrKeys {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }

    const BOUND: Bound = Bound::Bounded {
        max_size: 4,
        is_fixed_size: true,
    };
}

impl From<u32> for AttrKeys {
    fn from(num: u32) -> Self {
        let mut result = num.to_be_bytes();
        result.reverse();
        AttrKeys(result)
    }
}

impl From<AttrKeys> for u32 {
    fn from(keys: AttrKeys) -> Self {
        let mut result = keys.0;
        result.reverse();
        u32::from_be_bytes(result)
    }
}

impl Display for AttrKeys {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

impl AttrKeys {
    pub fn new(attr1: AttrKey, attr2: AttrKey, attr3: AttrKey, attr4: AttrKey) -> AttrKeys {
        AttrKeys([attr1, attr2, attr3, attr4])
    }

    pub fn get(&self, index: usize) -> AttrKey {
        self.0[index]
    }

    pub fn replace(&self, index: usize, attr: &AttrKey) -> Result<AttrKeys, Error> {
        if index > 3 {
            return Err(anyhow::anyhow!("Index out of range"));
        }
        let mut keys = self.0;
        keys[index] = *attr;
        Ok(AttrKeys(keys))
    }
}

impl FromStr for AttrKeys {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let result: AttrKeys = u32::from_str_radix(s, 16)?.into();
        Ok(result)
    }
}

/// The part of attributes in the item page.
pub type AttrIndexesResponse = [Option<AttrIndexResponse>; 4];

pub type AttrStatusesResponse = [Vec<Option<AttrStatusResponse>>; 4];

#[derive(PartialEq, Debug, CandidType, Serialize, Deserialize, Clone)]
pub struct AttrIndexResponse {
    pub name: String,
    pub value: Vec<AttrType>,
}

#[derive(PartialEq, Debug, CandidType, Serialize, Deserialize, Clone)]
pub enum AttrType {
    Text(String),
    Color(AttrColor),
}

#[derive(PartialEq, Debug, CandidType, Serialize, Deserialize, Clone)]
pub struct AttrColor {
    pub name: String,
    /// (Red, Green, Blue)
    pub color: (u8, u8, u8),
}

#[derive(PartialEq, Debug, CandidType, Serialize, Deserialize, Clone, Copy)]
pub struct AttrStatusResponse {
    pub is_in_stock: bool,
}

pub type Stock = u32;

#[derive(PartialEq, Eq, Hash, Debug, CandidType, Serialize, Deserialize, Clone, Copy)]
pub struct AttrRequest {
    pub keys: AttrKeys,
    pub changed_key_index: Option<u8>,
}
