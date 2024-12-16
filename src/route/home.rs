use crate::{market::MarketDataResponseWithItemGlances, unit::Currency};
use candid::{CandidType, Principal};
use serde::{Deserialize, Serialize};

#[derive(thiserror::Error, Debug, CandidType, Clone, Serialize, Deserialize, PartialEq)]
pub enum HomePageError {
    #[error("account (key: {0}) not found (AccontPageError::AccountNotFound)")]
    AccountNotFound(Principal),
}

#[derive(CandidType, Clone, Serialize, Deserialize, Debug, Hash, Eq, PartialEq)]
pub struct HomePageRequest {
    pub currency: Currency,
}

#[derive(CandidType, Clone, Serialize, Deserialize, Debug)]
pub struct HomePageResponse {
    pub markets: Vec<MarketDataResponseWithItemGlances>,
}
