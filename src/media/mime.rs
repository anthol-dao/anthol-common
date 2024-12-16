use candid::CandidType;
use serde::{Deserialize, Serialize};
use std::fmt;

mod audio;
mod image;
mod video;

pub use audio::MimeAudio;
pub use image::MimeImage;
pub use video::MimeVideo;

#[derive(CandidType, Clone, Debug, Serialize, Deserialize, Hash, Eq, PartialEq)]
pub enum Mime {
    Other(String),
    Image(MimeImage),
    Video(MimeVideo),
    Audio(MimeAudio),
}

impl fmt::Display for Mime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Mime::Other(other) => write!(f, "{}", other),
            Mime::Image(subtype) => write!(f, "image/{}", subtype),
            Mime::Video(subtype) => write!(f, "video/{}", subtype),
            Mime::Audio(subtype) => write!(f, "audio/{}", subtype),
        }
    }
}
