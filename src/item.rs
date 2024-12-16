use crate::{
    market::{MarketId, MarketName},
    media::MediaData,
    store::{StoreId, StoreName},
    unit::{Currency, Price},
};
use candid::{CandidType, Decode, Encode};
use ic_cdk::api::call::RejectionCode;
use ic_stable_structures::{storable::Bound, Storable};
use serde::{Deserialize, Serialize};
use std::{borrow::Cow, collections::BTreeMap};

pub mod attr;
mod id;
mod key;
pub mod spec;
pub mod tag;

use attr::{AttrIndexesResponse, AttrKeys, AttrRequest, AttrStatusesResponse, Stock};
pub use id::ItemId;
pub use key::ItemKey;
use spec::SpecResponse;
pub use tag::{Tag, TagError};

pub type ItemName = String;

/// Type of the item.
#[derive(CandidType, Copy, Clone, Debug, Serialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum ItemType {
    Physical,
    Digital,
}

#[derive(
    CandidType, Clone, Serialize, Deserialize, Debug, Hash, Eq, PartialEq, PartialOrd, Ord,
)]
pub struct IdentifiedItem {
    pub market_id: MarketId,
    pub store_id: StoreId,
    pub item_id: ItemId,
    pub attr_keys: AttrKeys,
}

#[derive(CandidType, Clone, Serialize, Deserialize, Debug, Hash, Eq, PartialEq)]
pub struct ItemPageRequest {
    pub market_id: MarketId,
    pub store_id: StoreId,
    pub request_to_store: ItemPageRequestToStoreCanister,
}

#[derive(CandidType, Clone, Serialize, Deserialize, Debug, Hash, Eq, PartialEq)]
pub struct ItemPageRequestToStoreCanister {
    pub item_id: ItemId,
    pub attr: AttrRequest,
    pub currency: Currency,
}

#[derive(CandidType, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct ItemPageResponse {
    pub static_data: Option<ItemPageStaticData>,
    pub price: Price,
    pub images: Vec<MediaDataWithCaption>,
    pub stock: Stock,
    pub attr_status: AttrStatusesResponse,
    pub specs: Option<SpecResponse>,
    pub fallback_attr: Option<AttrKeys>,
}
pub fn get_item_page_response(
    item_page_response: ItemPageResponseFromStoreCanister,
    market_name: MarketName,
) -> ItemPageResponse {
    ItemPageResponse {
        static_data: item_page_response
            .static_data
            .map(|static_data| ItemPageStaticData {
                market_name,
                data_from_store: static_data,
            }),
        price: item_page_response.price,
        images: item_page_response.images,
        stock: item_page_response.stock,
        attr_status: item_page_response.attr_status,
        specs: item_page_response.specs,
        fallback_attr: item_page_response.fallback_attr,
    }
}

#[derive(CandidType, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct ItemPageResponseFromStoreCanister {
    pub static_data: Option<ItemPageStaticDataFromStoreCanister>,
    pub price: Price,
    pub images: Vec<MediaDataWithCaption>,
    pub stock: Stock,
    pub attr_status: AttrStatusesResponse,
    pub specs: Option<SpecResponse>,
    pub fallback_attr: Option<AttrKeys>,
}

#[derive(CandidType, Clone, Debug, Serialize, Deserialize, PartialEq, Default)]
pub struct ItemPageStaticData {
    pub market_name: MarketName,
    pub data_from_store: ItemPageStaticDataFromStoreCanister,
}

#[derive(CandidType, Clone, Debug, Serialize, Deserialize, PartialEq, Default)]
pub struct ItemPageStaticDataFromStoreCanister {
    pub item_name: ItemName,
    pub descriptions: Vec<String>,
    pub tags: Vec<Tag>,
    pub attrs: AttrIndexesResponse,
    pub store_name: StoreName,
}

#[derive(CandidType, Copy, Clone, Debug, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum ItemPageErrorCode {
    ItemNotFound,
    NoAvailableAttr,
    StoreNotFound,
    MarketNotFound,
    StoreCanisterError(RejectionCode),
}

#[derive(CandidType, Clone, Debug, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct ItemPageError(pub ItemPageErrorCode, pub String);

#[derive(
    CandidType, Copy, Clone, Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord,
)]
pub enum ItemPageFromStoreErrorCode {
    ItemNotFound,
    NoAvailableAttr,
}

impl From<(ItemPageFromStoreErrorCode, String)> for ItemPageError {
    fn from((error_code, message): (ItemPageFromStoreErrorCode, String)) -> Self {
        ItemPageError(
            match error_code {
                ItemPageFromStoreErrorCode::ItemNotFound => ItemPageErrorCode::ItemNotFound,
                ItemPageFromStoreErrorCode::NoAvailableAttr => ItemPageErrorCode::NoAvailableAttr,
            },
            message,
        )
    }
}

impl From<(RejectionCode, String)> for ItemPageError {
    fn from((rejection_code, message): (RejectionCode, String)) -> Self {
        ItemPageError(
            ItemPageErrorCode::StoreCanisterError(rejection_code),
            message,
        )
    }
}

#[derive(CandidType, Clone, Debug, Serialize, Deserialize, Hash, Eq, PartialEq)]
pub struct MediaDataWithCaption {
    pub data: MediaData,
    pub caption: String,
}

impl MediaDataWithCaption {
    pub fn builder() -> MediaDataWithCaptionBuilder {
        MediaDataWithCaptionBuilder::default()
    }
}

#[derive(Default)]
pub struct MediaDataWithCaptionBuilder {
    pub data: Option<MediaData>,
    pub caption: Option<String>,
}

impl MediaDataWithCaptionBuilder {
    pub fn data(mut self, data: MediaData) -> Self {
        self.data = Some(data);
        self
    }

    pub fn caption(mut self, caption: &str) -> Self {
        self.caption = Some(caption.to_string());
        self
    }

    pub fn build(self) -> MediaDataWithCaption {
        MediaDataWithCaption {
            data: self.data.unwrap(),
            caption: self.caption.unwrap_or_default(),
        }
    }
}

#[derive(
    CandidType, Clone, Serialize, Deserialize, Debug, Hash, Eq, PartialOrd, Ord, PartialEq,
)]
pub struct ItemCoreKeys {
    pub item_id: ItemId,
    pub attr_keys: AttrKeys,
}

#[derive(CandidType, Clone, Serialize, Deserialize, Debug, Hash, Eq, PartialEq)]
pub struct ItemCoreDataRequest {
    pub item: ItemCoreKeys,
    pub currency: Currency,
}

#[derive(CandidType, Clone, Serialize, Deserialize, Debug, PartialEq)]
pub enum ItemDataInMarket {
    V1(ItemDataInMarketV1),
}

impl Storable for ItemDataInMarket {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }

    const BOUND: Bound = Bound::Unbounded;
}

#[derive(CandidType, Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct ItemDataInMarketV1 {
    pub item_name: String,
    pub store_name: String,
    pub tags: Vec<Tag>,
    pub attrs: Vec<(AttrKeys, ItemAttrSpecificDataInMarket)>,
}

#[derive(CandidType, Clone, Serialize, Deserialize, Debug, PartialEq)]
pub enum ItemAttrSpecificDataInMarket {
    V1(ItemAttrSpecificDataInMarketV1),
}

#[derive(CandidType, Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct ItemAttrSpecificDataInMarketV1 {
    pub is_in_stock: bool,
    pub price: BTreeMap<Currency, Price>,
    pub image: MediaData,
}

/// Summarized data for items in the market row, suggestion row, etc.
#[derive(CandidType, Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct ItemGlanceData {
    pub store_id: StoreId,
    pub item_id: ItemId,
    pub attr_keys: AttrKeys,
    pub item_name: String,
    pub store_name: String,
    pub tags: Vec<Tag>,
    pub is_in_stock: bool,
    pub price: Price,
    pub image: MediaData,
}

pub fn get_item_glance_data(
    store_id: &StoreId,
    item_id: &ItemId,
    data: &ItemDataInMarket,
    currency: &Currency,
) -> ItemGlanceData {
    match data {
        ItemDataInMarket::V1(data) => {
            let (attr_keys, item_attr_data) = data.attrs.first().unwrap();

            let (is_in_stock, price, image) = match item_attr_data {
                ItemAttrSpecificDataInMarket::V1(data) => (
                    data.is_in_stock,
                    data.price.get(currency).cloned().unwrap(),
                    data.image.clone(),
                ),
            };

            ItemGlanceData {
                store_id: *store_id,
                item_id: *item_id,
                attr_keys: *attr_keys,
                item_name: data.item_name.clone(),
                store_name: data.store_name.clone(),
                tags: data.tags.clone(),
                is_in_stock,
                price,
                image,
            }
        }
    }
}
