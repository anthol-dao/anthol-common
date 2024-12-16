use candid::CandidType;
use derive_more::{AsRef, Display, From, Into};
use ic_cdk::api::{call::CallResult, management_canister::main::raw_rand};
use ic_stable_structures::{storable::Bound, Storable};
use serde::{Deserialize, Serialize};

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
    Into,
)]
pub struct ItemKey(u64);

impl ItemKey {
    pub async fn create() -> CallResult<Self> {
        Ok(Self(Self::get_random_u64().await?))
    }

    pub async fn create_four() -> CallResult<[Self; 4]> {
        let random_u64s = Self::get_four_random_u64().await?;
        Ok([
            Self(random_u64s[0]),
            Self(random_u64s[1]),
            Self(random_u64s[2]),
            Self(random_u64s[3]),
        ])
    }

    async fn get_random_u64() -> CallResult<u64> {
        let random_bytes = Self::get_random_32_bytes().await?;
        let mut random_u64 = 0u64;
        for (i, &byte) in random_bytes.iter().take(8).enumerate() {
            random_u64 |= (byte as u64) << (i * 8);
        }

        Ok(random_u64)
    }

    async fn get_four_random_u64() -> CallResult<[u64; 4]> {
        let random_bytes = Self::get_random_32_bytes().await?;
        let mut random_u64 = [0u64; 4];
        for (i, &byte) in random_bytes.iter().enumerate() {
            random_u64[i / 8] |= (byte as u64) << ((i % 8) * 8);
        }

        Ok(random_u64)
    }

    async fn get_random_32_bytes() -> CallResult<[u8; 32]> {
        let (random_bytes,) = raw_rand().await?;
        Ok(random_bytes.as_slice().try_into().unwrap())
    }
}

impl Storable for ItemKey {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        self.0.to_bytes()
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Self(u64::from_bytes(bytes))
    }

    const BOUND: Bound = Bound::Bounded {
        max_size: 8,
        is_fixed_size: true,
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_item_key() {
        let key = ItemKey(9525480706759653391);
        let bytes = key.to_bytes();
        let key2 = ItemKey::from_bytes(bytes);
        assert_eq!(key, key2);
    }

    #[test]
    fn test_item_key_bytes_length() {
        let key = ItemKey(1419349344772532485);
        let bytes = key.to_bytes();
        assert_eq!(bytes.len(), 8);
    }
}
