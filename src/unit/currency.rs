use candid::CandidType;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(
    CandidType, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Debug, Hash, Copy,
)]
pub enum Currency {
    USD,
    CNY,
    JPY,
    EUR,
    GBP,
    BTC,
    ETH,
    ICP,
    USDT,
    USDC,
    FLOS,
}

impl fmt::Display for Currency {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Currency::USD => "USD",
                Currency::CNY => "CNY",
                Currency::JPY => "JPY",
                Currency::EUR => "EUR",
                Currency::GBP => "GBP",
                Currency::BTC => "BTC",
                Currency::ETH => "ETH",
                Currency::ICP => "ICP",
                Currency::USDT => "USDT",
                Currency::USDC => "USDC",
                Currency::FLOS => "FLOS",
            }
        )
    }
}
