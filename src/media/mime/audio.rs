use candid::CandidType;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(CandidType, Clone, Debug, Serialize, Deserialize, Hash, Eq, PartialEq)]
pub enum MimeAudio {
    Other(String),
    Aac,
    Mp3,
    Ogg,
    Wav,
    Webm,
    Flac,
    Alac,
    Aiff,
    Opus,
    Mp4,
}

impl fmt::Display for MimeAudio {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MimeAudio::Other(other) => write!(f, "{}", other),
            MimeAudio::Aac => write!(f, "aac"),
            MimeAudio::Mp3 => write!(f, "mp3"),
            MimeAudio::Ogg => write!(f, "ogg"),
            MimeAudio::Wav => write!(f, "wav"),
            MimeAudio::Webm => write!(f, "webm"),
            MimeAudio::Flac => write!(f, "flac"),
            MimeAudio::Alac => write!(f, "alac"),
            MimeAudio::Aiff => write!(f, "aiff"),
            MimeAudio::Opus => write!(f, "opus"),
            MimeAudio::Mp4 => write!(f, "mp4"),
        }
    }
}
