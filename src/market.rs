use crate::{
    id::{Id, IdError},
    item::{ItemCoreKeys, ItemGlanceData},
};
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
pub struct MarketPrincipal(Principal);

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
pub struct MarketId(Id);

impl MarketId {
    pub fn new<T: AsRef<str>>(id: T) -> Result<Self, IdError> {
        Ok(MarketId(Id::new(id)?))
    }
}

impl FromStr for MarketId {
    type Err = IdError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(MarketId(Id::from_str(s)?))
    }
}

impl TryFrom<&str> for MarketId {
    type Error = IdError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(MarketId(Id::from_str(value)?))
    }
}

impl TryFrom<String> for MarketId {
    type Error = IdError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Ok(MarketId(Id::from_str(&value)?))
    }
}

impl Storable for MarketPrincipal {
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

impl Storable for MarketId {
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

pub type MarketName = String;

#[derive(CandidType, Clone, Serialize, Deserialize, Debug)]
pub struct MarketDataResponse {
    pub name: MarketName,
    pub items: Vec<ItemCoreKeys>,
}

#[derive(CandidType, Clone, Serialize, Deserialize, Debug)]
pub struct MarketDataResponseWithItemGlances {
    pub id: MarketId,
    pub name: MarketName,
    pub items: Vec<ItemGlanceData>,
}

#[derive(CandidType, Clone, Serialize, Deserialize, Debug)]
pub struct MarketInitArg {
    pub id: MarketId,
    pub name: MarketName,
}
