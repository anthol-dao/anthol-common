use crate::util::string::adjust_whitespaces;
use candid::{CandidType, Decode, Encode};
use heck::ToTitleCase;
use ic_stable_structures::{storable::Bound, Storable};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::fmt;

const INVALID_SYMBOLS: &str = r##"!\"#$%&'()*+,-./:;<=>?@[\]^`{|}~"##;

/// A tag is a string that is used to categorize items.
/// Tags are case insensitive and displayed in title case.
#[derive(
    CandidType, Clone, Debug, Serialize, Deserialize, Hash, Eq, PartialEq, PartialOrd, Ord,
)]
pub struct Tag(String);

impl Tag {
    /// Create a new tag.
    /// The tag is converted to lowercase and limited to 48 characters.
    /// When displayed, it is converted to title case.
    ///
    /// # Errors
    /// Returns an error if the tag is empty, too long, or contains ascii symbols except underscore(_).
    pub fn new<T: AsRef<str>>(tag: T) -> Result<Self, TagCreationError> {
        let tag = adjust_whitespaces(tag.as_ref().trim());

        if tag.is_empty() {
            return Err(TagCreationError::TagEmpty(tag.to_string()));
        }

        if tag.len() > 48 {
            return Err(TagCreationError::TagTooLong(tag.to_string()));
        }

        if tag.chars().any(|c: char| INVALID_SYMBOLS.contains(c)) {
            return Err(TagCreationError::TagInvalidCharacters(tag.to_string()));
        }

        Ok(Tag(tag.to_lowercase()))
    }

    /// Convert the tag to a URL encoded string.
    /// Spaces are converted to hyphens.
    pub fn to_url(&self) -> String {
        let str = self.0.replace(' ', "-");
        urlencoding::encode(str.as_str()).into_owned()
    }
}

impl AsRef<str> for Tag {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for Tag {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0.to_title_case())
    }
}

impl Storable for Tag {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }

    const BOUND: Bound = Bound::Bounded {
        max_size: 192,
        is_fixed_size: false,
    };
}

#[derive(thiserror::Error, Debug, CandidType, Clone, Serialize, Deserialize, PartialEq)]
pub enum TagCreationError {
    #[error("Tag ({0}) is too long")]
    TagTooLong(String),
    #[error("Tag ({0}) is empty")]
    TagEmpty(String),
    #[error("Tag ({0}) contains invalid characters")]
    TagInvalidCharacters(String),
    #[error("Tag ({0}) already exists")]
    TagExists(String),
}

#[derive(thiserror::Error, Debug, CandidType, Clone, Serialize, Deserialize, PartialEq)]
pub enum TagError {
    #[error("Tag ({0}) not found")]
    TagNotFound(Tag),
}
