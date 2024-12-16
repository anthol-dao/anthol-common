use crate::{
    item::{
        attr::{AttrIndexesResponse, AttrKeys, Stock},
        ItemId, ItemName,
    },
    market::{MarketId, MarketName},
    media::MediaData,
    store::{StoreId, StoreName},
    unit::Price,
};
use candid::CandidType;
use serde::{Deserialize, Serialize};

#[derive(CandidType, Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct UserBasketPageResponse {
    pub physical_items: Vec<PhysicalItemGroupOfSender>,
    pub digital_items:
        Vec<Result<DigitalItemInBasket, (BasketItemError, Option<DigitalItemInBasket>)>>,
}

#[derive(CandidType, Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct PhysicalItemGroupOfSender {
    pub store_id: StoreId,
    pub store_name: StoreName,
    pub items: Vec<Result<PhysicalItemInBasket, (BasketItemError, Option<PhysicalItemInBasket>)>>,
}

#[derive(CandidType, Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct PhysicalItemInBasket {
    pub market_id: MarketId,
    pub market_name: MarketName,
    pub item_id: ItemId,
    pub item_name: ItemName,
    pub image: MediaData,
    pub attr_keys: AttrKeys,
    pub attrs: AttrIndexesResponse,
    pub price: Price,
    pub count: u32,
    pub stock: Stock,
}

#[derive(CandidType, Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct DigitalItemInBasket {
    pub market_id: MarketId,
    pub market_name: MarketName,
    pub store_id: StoreId,
    pub store_name: StoreName,
    pub item_id: ItemId,
    pub item_name: ItemName,
    pub image: MediaData,
    pub attr_keys: AttrKeys,
    pub attrs: AttrIndexesResponse,
    pub price: Price,
    pub count: u32,
    pub stock: Stock,
}

#[derive(CandidType, Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ItemInBasket {
    pub market_id: MarketId,
    pub market_name: MarketName,
    pub store_id: StoreId,
    pub store_name: StoreName,
    pub item_id: ItemId,
    pub item_name: ItemName,
    pub image: MediaData,
    pub attr_keys: AttrKeys,
    pub attrs: AttrIndexesResponse,
    pub price: Price,
    pub count: u32,
    pub stock: Stock,
}

impl From<((StoreId, StoreName), PhysicalItemInBasket)> for ItemInBasket {
    fn from(item: ((StoreId, StoreName), PhysicalItemInBasket)) -> Self {
        ItemInBasket {
            market_id: item.1.market_id,
            market_name: item.1.market_name,
            store_id: item.0 .0,
            store_name: item.0 .1,
            item_id: item.1.item_id,
            item_name: item.1.item_name,
            image: item.1.image,
            attr_keys: item.1.attr_keys,
            attrs: item.1.attrs,
            price: item.1.price,
            count: item.1.count,
            stock: item.1.stock,
        }
    }
}

impl From<DigitalItemInBasket> for ItemInBasket {
    fn from(item: DigitalItemInBasket) -> Self {
        ItemInBasket {
            market_id: item.market_id,
            market_name: item.market_name,
            store_id: item.store_id,
            store_name: item.store_name,
            item_id: item.item_id,
            item_name: item.item_name,
            image: item.image,
            attr_keys: item.attr_keys,
            attrs: item.attrs,
            price: item.price,
            count: item.count,
            stock: item.stock,
        }
    }
}

#[derive(CandidType, Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum BasketItemError {
    ItemNotFound,
    ItemOutOfStock,
    QuantityNotEnough,
    MarketNotFound,
    StoreNotFound,
}

#[derive(CandidType, Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum UserBasketPageError {
    ActorNotFound,
    ActorIsNotUser,
}
