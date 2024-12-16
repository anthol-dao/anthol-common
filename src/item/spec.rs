use candid::{CandidType, Deserialize};
use serde::Serialize;

pub type SpecValue = Vec<String>;

pub type SpecResponse = Vec<SpecResponseCategory>;

#[derive(CandidType, Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct SpecResponseCategory {
    pub category_name: String,
    pub label_vec: Vec<SpecResponseLabel>,
}

#[derive(CandidType, Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct SpecResponseLabel {
    pub label_name: String,
    pub value: SpecValue,
}
