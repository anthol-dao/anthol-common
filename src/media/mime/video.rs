use candid::CandidType;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(CandidType, Clone, Debug, Serialize, Deserialize, Hash, Eq, PartialEq)]
pub enum MimeVideo {
    Other(String),
    Mp4,
    Av1,
    Mpeg,
    Ogg,
    Quicktime,
    Webm,
    Vp8,
    Vp9,
    H264,
    H265,
}

impl fmt::Display for MimeVideo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MimeVideo::Other(other) => write!(f, "{}", other),
            MimeVideo::Mp4 => write!(f, "mp4"),
            MimeVideo::Av1 => write!(f, "AV1"),
            MimeVideo::Mpeg => write!(f, "mpeg"),
            MimeVideo::Ogg => write!(f, "ogg"),
            MimeVideo::Quicktime => write!(f, "quicktime"),
            MimeVideo::Webm => write!(f, "webm"),
            MimeVideo::Vp8 => write!(f, "VP8"),
            MimeVideo::Vp9 => write!(f, "VP9"),
            MimeVideo::H264 => write!(f, "H264"),
            MimeVideo::H265 => write!(f, "H265"),
        }
    }
}
