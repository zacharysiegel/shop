use crate::error::ShopError;
use crate::item::ItemStatus;
use crate::server::JsonHttpResponse;
use crate::{ShopEntity, ShopModel, ShopSerial};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug)]
pub struct ItemAuditEntity {
    pub id: Uuid,
    pub item_id: Uuid,
    pub status_before: i32,
    pub status_after: i32,
    pub initiated_by_admin: bool,
    pub note: Option<String>,
    pub created: DateTime<Utc>,
}

impl ShopEntity for ItemAuditEntity {
    type Model = ItemAuditModel;
}

#[derive(Debug)]
pub struct ItemAuditModel {
    pub id: Uuid,
    pub item_id: Uuid,
    pub status_before: ItemStatus,
    pub status_after: ItemStatus,
    pub initiated_by_admin: bool,
    pub note: Option<String>,
    pub created: DateTime<Utc>,
}

impl ShopModel for ItemAuditModel {
    type Entity = ItemAuditEntity;
    type Serial = ItemAuditSerial;

    fn to_serial(&self) -> Self::Serial {
        ItemAuditSerial {
            id: self.id.clone(),
            item_id: self.item_id.clone(),
            status_before: self.status_before.clone() as u8,
            status_after: self.status_after.clone() as u8,
            initiated_by_admin: self.initiated_by_admin.clone(),
            note: self.note.clone(),
            created: self.created.clone(),
        }
    }

    fn try_from_serial(serial: &Self::Serial) -> Result<Self, ShopError> {
        Ok(ItemAuditModel {
            id: serial.id.clone(),
            item_id: serial.item_id.clone(),
            status_before: ItemStatus::try_from_with_shoperror(serial.status_before)?,
            status_after: ItemStatus::try_from_with_shoperror(serial.status_after)?,
            initiated_by_admin: serial.initiated_by_admin.clone(),
            note: serial.note.clone(),
            created: serial.created.clone(),
        })
    }

    fn to_entity(&self) -> Self::Entity {
        ItemAuditEntity {
            id: self.id.clone(),
            item_id: self.item_id.clone(),
            status_before: i32::from(self.status_before.clone() as u8),
            status_after: i32::from(self.status_after.clone() as u8),
            initiated_by_admin: self.initiated_by_admin.clone(),
            note: self.note.clone(),
            created: self.created.clone(),
        }
    }

    fn try_from_entity(entity: &Self::Entity) -> Result<Self, ShopError> {
        Ok(ItemAuditModel {
            id: entity.id.clone(),
            item_id: entity.item_id.clone(),
            status_before: ItemStatus::try_from_with_shoperror(entity.status_before as u8)?,
            status_after: ItemStatus::try_from_with_shoperror(entity.status_after as u8)?,
            initiated_by_admin: entity.initiated_by_admin.clone(),
            note: entity.note.clone(),
            created: entity.created.clone(),
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ItemAuditSerial {
    pub id: Uuid,
    pub item_id: Uuid,
    pub status_before: u8,
    pub status_after: u8,
    pub initiated_by_admin: bool,
    pub note: Option<String>,
    pub created: DateTime<Utc>,
}

impl ShopSerial for ItemAuditSerial {
    type Model = ItemAuditModel;
}

impl JsonHttpResponse for ItemAuditSerial {}
impl JsonHttpResponse for Vec<ItemAuditSerial> {}
