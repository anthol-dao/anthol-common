use candid::CandidType;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(CandidType, Clone, Debug, Serialize, Deserialize, Hash, Eq, PartialEq)]
pub enum MimeImage {
    Other(String),
    Gif,
    Jpeg,
    Png,
    Svg,
    Tiff,
    Webp,
    Apng,
    Avif,
    Heif,
}

impl fmt::Display for MimeImage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MimeImage::Other(other) => write!(f, "{}", other),
            MimeImage::Gif => write!(f, "gif"),
            MimeImage::Jpeg => write!(f, "jpeg"),
            MimeImage::Png => write!(f, "png"),
            MimeImage::Svg => write!(f, "svg+xml"),
            MimeImage::Tiff => write!(f, "tiff"),
            MimeImage::Webp => write!(f, "webp"),
            MimeImage::Apng => write!(f, "apng"),
            MimeImage::Avif => write!(f, "avif"),
            MimeImage::Heif => write!(f, "heif"),
        }
    }
}
