use crate::error::ShopError;
use crate::object::JsonHttpResponse;
use crate::{create_json_spec, object, try_from_repr, ShopEntity, ShopModel, ShopSerial};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fmt;
use std::fmt::Display;
use strum::{FromRepr, IntoStaticStr, VariantArray};
use uuid::Uuid;

#[derive(Debug)]
pub struct Item {
    pub id: Uuid,
    pub product_id: Uuid,
    pub inventory_location_id: Uuid,
    pub condition: ItemCondition,
    pub status: ItemStatus,
    pub price_cents: u32,
    pub priority: i32,
    pub note: Option<String>,
    pub acquisition_datetime: DateTime<Utc>,
    pub acquisition_price_cents: Option<u32>,
    pub acquisition_location: Option<String>,
    pub created: DateTime<Utc>,
    pub updated: DateTime<Utc>,
    // Add grading information?
    // Add refurbishment information?
}

/// Variants inspired by Ebay: https://www.ebay.com/help/selling/listings/creating-managing-listings/item-conditions-category.
/// See the "Movies & TV, Music, Video Games" section.
#[derive(Debug, Clone, FromRepr, VariantArray, IntoStaticStr)]
#[repr(u8)]
pub enum ItemCondition {
    Inapplicable = 0,
    BrandNew,
    LikeNew,
    VeryGood,
    Good,
    Acceptable,
    Digital,
}

impl ItemCondition {
    pub fn to_serial(&self) -> &'static str {
        match self {
            ItemCondition::Inapplicable => "inapplicable",
            ItemCondition::BrandNew => "brand_new",
            ItemCondition::LikeNew => "like_new",
            ItemCondition::VeryGood => "very_good",
            ItemCondition::Good => "good",
            ItemCondition::Acceptable => "acceptable",
            ItemCondition::Digital => "digital",
        }
    }
}

impl Display for ItemCondition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ({})", Into::<&'static str>::into(self), self.clone() as u8)
    }
}

create_json_spec!(ItemCondition<u8>);
try_from_repr!(ItemCondition<u8>);

#[derive(Debug, Clone, FromRepr, VariantArray, IntoStaticStr)]
#[repr(u8)]
pub enum ItemStatus {
    /// Item is only partially constructed and expects modifications before publishing
    Incomplete = 0,
    /// Item is completely specified but is not yet listed on any marketplace
    CompleteUnlisted,
    /// Item is listed on any marketplace
    CompleteListed,
    /// A customer has requested the item be delisted from all marketplaces in anticipation of purchase.
    CustomerHoldListed,
    /// Item has been delisted from all marketplaces at the request of a potential buyer
    CustomerHoldDelisted,
    /// Item has been purchased but has not yet been delisted from all marketplaces
    PurchaseListed,
    /// Item has been delisted from all marketplaces after a purchase
    PurchasedDelisted,
    /// Item has been shipped
    Shipped,
    /// Item has been received by the customer (either via shipping or pickup)
    Received,
}

impl Display for ItemStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ({})", Into::<&'static str>::into(self), self.clone() as u8)
    }
}

try_from_repr!(ItemStatus<u8>);

impl ShopModel for Item {
    type Entity = ItemEntity;
    type Serial = ItemSerial;

    fn to_serial(&self) -> Self::Serial {
        ItemSerial {
            id: self.id.clone(),
            product_id: self.product_id.clone(),
            inventory_location_id: self.inventory_location_id.clone(),
            condition: self.condition.clone() as u8,
            status: self.status.clone() as u8,
            price_cents: self.price_cents.clone(),
            priority: self.priority.clone(),
            note: self.note.clone(),
            acquisition_datetime: self.acquisition_datetime.clone(),
            acquisition_price_cents: self.acquisition_price_cents.clone(),
            acquisition_location: self.acquisition_location.clone(),
            created: self.created.clone(),
            updated: self.updated.clone(),
        }
    }

    fn try_from_serial(serial: &Self::Serial) -> Result<Self, ShopError> {
        Ok(Item {
            id: object::random_uuid(),
            product_id: serial.product_id.clone(),
            inventory_location_id: serial.inventory_location_id.clone(),
            condition: ItemCondition::try_from_repr(serial.condition)?,
            status: ItemStatus::try_from_repr(serial.status)?,
            price_cents: serial.price_cents.clone(),
            priority: serial.priority.clone(),
            note: serial.note.clone(),
            acquisition_datetime: serial.acquisition_datetime.clone(),
            acquisition_price_cents: serial.acquisition_price_cents.clone(),
            acquisition_location: serial.acquisition_location.clone(),
            created: serial.created.clone(),
            updated: serial.updated.clone(),
        })
    }

    fn to_entity(&self) -> Self::Entity {
        ItemEntity {
            id: self.id.clone(),
            product_id: self.product_id.clone(),
            inventory_location_id: self.inventory_location_id.clone(),
            condition: self.condition.clone() as u8 as i32,
            status: self.status.clone() as u8 as i32,
            price_cents: i64::from(self.price_cents),
            priority: self.priority.clone(),
            note: self.note.clone(),
            acquisition_datetime: self.acquisition_datetime.clone(),
            acquisition_price_cents: self.acquisition_price_cents.map(|v| i64::from(v)),
            acquisition_location: self.acquisition_location.clone(),
            created: self.created.clone(),
            updated: self.updated.clone(),
        }
    }

    fn try_from_entity(entity: &Self::Entity) -> Result<Self, ShopError> {
        Ok(Item {
            id: entity.id.clone(),
            product_id: entity.product_id.clone(),
            inventory_location_id: entity.inventory_location_id.clone(),
            condition: ItemCondition::try_from_repr(entity.condition as u8)?,
            status: ItemStatus::try_from_repr(entity.status as u8)?,
            price_cents: entity.price_cents.clone() as u32,
            priority: entity.priority.clone(),
            note: entity.note.clone(),
            acquisition_datetime: entity.acquisition_datetime.clone(),
            acquisition_price_cents: entity.acquisition_price_cents.clone().map(|v| v as u32),
            acquisition_location: entity.acquisition_location.clone(),
            created: entity.created.clone(),
            updated: entity.updated.clone(),
        })
    }
}

#[derive(Debug)]
pub struct ItemEntity {
    pub id: Uuid,
    pub product_id: Uuid,
    pub inventory_location_id: Uuid,
    pub condition: i32,
    pub status: i32,
    pub price_cents: i64,
    pub priority: i32,
    pub note: Option<String>,
    pub acquisition_datetime: DateTime<Utc>,
    pub acquisition_price_cents: Option<i64>,
    pub acquisition_location: Option<String>,
    pub created: DateTime<Utc>,
    pub updated: DateTime<Utc>,
}

impl ShopEntity for ItemEntity {
    type Model = Item;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ItemSerial {
    #[serde(default)]
    pub id: Uuid,
    pub product_id: Uuid,
    pub inventory_location_id: Uuid,
    pub condition: u8,
    pub status: u8,
    pub price_cents: u32,
    pub priority: i32,
    pub note: Option<String>,
    pub acquisition_datetime: DateTime<Utc>,
    pub acquisition_price_cents: Option<u32>,
    pub acquisition_location: Option<String>,
    pub created: DateTime<Utc>,
    pub updated: DateTime<Utc>,
}

impl ShopSerial for ItemSerial {
    type Model = Item;
}
impl JsonHttpResponse for ItemSerial {}
impl JsonHttpResponse for Vec<ItemSerial> {}
