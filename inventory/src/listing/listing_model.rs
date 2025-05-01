use crate::error::ShopError;
use crate::object::JsonHttpResponse;
use crate::{try_from_repr, object, ShopEntity, ShopModel, ShopSerial};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use strum::FromRepr;
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
    type Model = ListingModel;
}

#[derive(Debug)]
pub struct ListingModel {
    pub id: Uuid,
    pub item_id: Uuid,
    pub marketplace_id: Uuid,
    pub uri: Option<String>,
    pub status: ListingStatus,
    pub created: DateTime<Utc>,
    pub updated: DateTime<Utc>,
}

impl ShopModel for ListingModel {
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
        Ok(ListingModel {
            id: object::random_uuid(),
            item_id: serial.item_id.clone(),
            marketplace_id: serial.marketplace_id.clone(),
            uri: serial.uri.clone(),
            status: ListingStatus::try_from_repr(serial.status.clone())?,
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
        Ok(ListingModel {
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

#[derive(Debug, Clone, FromRepr)]
#[repr(u8)]
pub enum ListingStatus {
    Draft = 0,
    Published,
    Hold,
    Fulfilled,
    Cancelled,
}

try_from_repr!(ListingStatus<u8>);

#[derive(Debug, Serialize, Deserialize)]
pub struct ListingSerial {
    #[serde(default)]
    pub id: Uuid,
    pub item_id: Uuid,
    pub marketplace_id: Uuid,
    pub uri: Option<String>,
    pub status: u8,
    pub created: DateTime<Utc>,
    pub updated: DateTime<Utc>,
}

impl ShopSerial for ListingSerial {
    type Model = ListingModel;
}

impl JsonHttpResponse for ListingSerial {}
impl JsonHttpResponse for Vec<ListingSerial> {}
