use candid::CandidType;
use serde::{Deserialize, Serialize};

pub mod mime;

use mime::Mime;

#[derive(CandidType, Clone, Debug, Serialize, Deserialize, Hash, Eq, PartialEq)]
pub struct MediaData {
    pub src: MediaSrc,
    pub mime: Mime,
    pub alt: Option<String>,
}

impl MediaData {
    pub fn builder() -> MediaDataBuilder {
        MediaDataBuilder::default()
    }
}

#[derive(Default)]
pub struct MediaDataBuilder {
    pub src: Option<MediaSrc>,
    pub mime: Option<Mime>,
    pub alt: Option<String>,
}

impl MediaDataBuilder {
    pub fn url(mut self, url: &str) -> Self {
        self.src = Some(MediaSrc::URL(url.to_string()));
        self
    }

    pub fn cid(mut self, cid: &str) -> Self {
        self.src = Some(MediaSrc::CID(cid.to_string()));
        self
    }

    pub fn mime(mut self, mime: Mime) -> Self {
        self.mime = Some(mime);
        self
    }

    pub fn alt(mut self, alt: &str) -> Self {
        self.alt = Some(alt.to_string());
        self
    }

    pub fn build(self) -> MediaData {
        MediaData {
            src: self.src.unwrap(),
            mime: self.mime.unwrap(),
            alt: self.alt,
        }
    }
}

#[derive(CandidType, Clone, Debug, Serialize, Deserialize, Hash, Eq, PartialEq)]
pub enum MediaSrc {
    URL(String),
    CID(String),
}

impl MediaSrc {
    pub fn new_url() -> MediaSrc {
        MediaSrc::URL(String::new())
    }

    pub fn into_string(&self) -> String {
        match self {
            MediaSrc::URL(url) => url.clone(),
            MediaSrc::CID(cid) => cid.clone(),
        }
    }
}
