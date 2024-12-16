use crate::consts::IPFS_GATEWAY;
use candid::{CandidType, Principal};
use serde::{Deserialize, Serialize};

#[derive(CandidType, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum AccountPageResponse {
    User(UserAccountPageResponse),
}

#[derive(CandidType, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct UserAccountPageResponse {
    pub name: String,
    pub birth_name: String,
    pub mail_address: String,
    pub image: AccountImage,
}

impl UserAccountPageResponse {
    pub fn update_with_request(&mut self, request: &SetUserProfileRequest) {
        self.name.clone_from(&request.name);
        self.birth_name.clone_from(&request.birth_name);
        self.mail_address.clone_from(&request.mail_address);
        self.image.clone_from(&request.image);
    }
}

#[derive(thiserror::Error, Debug, CandidType, Clone, Serialize, Deserialize, PartialEq)]
pub enum AccountPageError {
    #[error("account (key: {0}) not found (AccontPageError::AccountNotFound)")]
    AccountNotFound(Principal),
}

#[derive(CandidType, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct SetUserProfileRequest {
    pub name: String,
    pub birth_name: String,
    pub mail_address: String,
    pub image: AccountImage,
}

impl SetUserProfileRequest {
    pub fn equal_to_response(&self, response: UserAccountPageResponse) -> bool {
        self.name == response.name
            && self.birth_name == response.birth_name
            && self.mail_address == response.mail_address
            && self.image == response.image
    }
}

impl From<UserAccountPageResponse> for SetUserProfileRequest {
    fn from(response: UserAccountPageResponse) -> Self {
        Self {
            name: response.name,
            birth_name: response.birth_name,
            mail_address: response.mail_address,
            image: response.image,
        }
    }
}

#[derive(thiserror::Error, Debug, CandidType, Clone, Serialize, Deserialize, PartialEq)]
pub enum SetUserProfileError {
    #[error("account (key: {0}) not found (AccontPageError::AccountNotFound)")]
    AccountNotFound(Principal),
    #[error("account (key: {0}) is not a user (SetUserProfileError::AccountIsNotUser)")]
    AccountIsNotUser(Principal),
}

#[derive(Debug, CandidType, Clone, Serialize, Deserialize, PartialEq)]
pub enum AccountImage {
    None,
    CID(String),
    BLOB(Vec<u8>),
}

impl AccountImage {
    pub fn to_url(&self) -> String {
        match self {
            AccountImage::None => "".to_string(),
            AccountImage::CID(cid) => format!("{}{}", IPFS_GATEWAY, cid),
            AccountImage::BLOB(_) => "".to_string(),
        }
    }

    pub fn is_none(&self) -> bool {
        self == &AccountImage::None
    }
}
