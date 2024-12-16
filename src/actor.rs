use candid::{CandidType, Principal};
use derive_more::{AsRef, Display, From, Into};
use ic_stable_structures::{storable::Bound, Storable};
use serde::{Deserialize, Serialize};

pub mod account;
pub mod id;

pub use id::{ActorId, ActorIdError};

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
pub struct ActorPrincipal(Principal);

impl Storable for ActorPrincipal {
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

/// New type for actor name up to 50 characters
#[derive(
    CandidType,
    Clone,
    Serialize,
    Deserialize,
    Debug,
    Default,
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
pub struct ActorName(String);

impl Storable for ActorName {
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

impl ActorName {
    pub fn new<T: AsRef<str>>(name: T) -> Result<Self, ActorNameError> {
        let name = name.as_ref();
        if name.chars().count() > 50 {
            return Err(ActorNameError::TooLong);
        }
        Ok(ActorName(name.to_string()))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ActorNameError {
    TooLong,
}
