use crate::error::ShopError;
use crate::object::JsonHttpResponse;
use crate::{object, try_from_repr, ShopEntity, ShopModel, ShopSerial};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use strum::{FromRepr, IntoStaticStr, VariantArray};
use uuid::Uuid;

#[derive(Debug)]
pub struct ListingEntity {
    pub id: Uuid,
    pub item_id: Uuid,
    pub marketplace_id: Uuid,
    pub uri: Option<String>,
    pub status: i32,
    pub created: DateTime<Utc>,
    pub updated: DateTime<Utc>,
}

impl ShopEntity for ListingEntity {
    type Model = Listing;
}

#[derive(Debug, Clone)]
pub struct Listing {
    pub id: Uuid,
    pub item_id: Uuid,
    pub marketplace_id: Uuid,
    pub uri: Option<String>,
    pub status: ListingStatus,
    pub created: DateTime<Utc>,
    pub updated: DateTime<Utc>,
}

impl ShopModel for Listing {
    type Entity = ListingEntity;
    type Serial = ListingSerial;

    fn to_serial(&self) -> Self::Serial {
        ListingSerial {
            id: self.id.clone(),
            item_id: self.item_id.clone(),
            marketplace_id: self.marketplace_id.clone(),
            uri: self.uri.clone(),
            status: self.status.clone() as u8,
            created: self.created.clone(),
            updated: self.updated.clone(),
        }
    }

    fn try_from_serial(serial: &Self::Serial) -> Result<Self, ShopError> {
        Ok(Listing {
            id: object::random_uuid(),
            item_id: serial.item_id.clone(),
            marketplace_id: serial.marketplace_id.clone(),
            uri: serial.uri.clone(),
            status: ListingStatus::default(),
            created: serial.created.clone(),
            updated: serial.updated.clone(),
        })
    }

    fn to_entity(&self) -> Self::Entity {
        ListingEntity {
            id: self.id.clone(),
            item_id: self.item_id.clone(),
            marketplace_id: self.marketplace_id.clone(),
            uri: self.uri.clone(),
            status: self.status.clone() as i32,
            created: self.created.clone(),
            updated: self.updated.clone(),
        }
    }

    fn try_from_entity(entity: &Self::Entity) -> Result<Self, ShopError> {
        Ok(Listing {
            id: entity.id.clone(),
            item_id: entity.item_id.clone(),
            marketplace_id: entity.marketplace_id.clone(),
            uri: entity.uri.clone(),
            status: ListingStatus::try_from_repr(entity.status.clone() as u8)?,
            created: entity.created.clone(),
            updated: entity.updated.clone(),
        })
    }
}

#[derive(Debug, Clone, PartialEq, FromRepr, VariantArray, IntoStaticStr)]
#[repr(u8)]
pub enum ListingStatus {
    Draft = 0,
    Published,
    Hold,
    Fulfilled,
    Cancelled,
}

try_from_repr!(ListingStatus<u8>);

impl Default for ListingStatus {
    fn default() -> Self {
        ListingStatus::Draft
    }
}

impl Display for ListingStatus {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({})", Into::<&'static str>::into(self), self.clone() as u8)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListingSerial {
    #[serde(default)]
    pub id: Uuid,
    pub item_id: Uuid,
    pub marketplace_id: Uuid,
    pub uri: Option<String>,
    #[serde(default)]
    pub status: u8,
    pub created: DateTime<Utc>,
    pub updated: DateTime<Utc>,
}

impl ShopSerial for ListingSerial {
    type Model = Listing;
}

impl JsonHttpResponse for ListingSerial {}
impl JsonHttpResponse for Vec<ListingSerial> {}
