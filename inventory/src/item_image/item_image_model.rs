use crate::error::ShopError;
use crate::object::JsonHttpResponse;
use crate::{object, ShopEntity, ShopModel, ShopSerial};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct ItemImageEntity {
    pub id: Uuid,
    pub item_id: Uuid,
    pub alt_text: String,
    pub priority: i32,
    pub original_file_name: String,
}

impl ShopEntity for ItemImageEntity {
    type Model = ItemImage;
}

pub type ItemImage = ItemImageEntity;

impl ShopModel for ItemImage {
    type Entity = ItemImageEntity;
    type Serial = ItemImageSerial;

    fn to_serial(&self) -> Self::Serial {
        ItemImageSerial {
            id: self.id.clone(),
            item_id: self.item_id.clone(),
            alt_text: self.alt_text.clone(),
            priority: self.priority.clone(),
            original_file_name: self.original_file_name.clone(),
        }
    }

    fn try_from_serial(serial: &Self::Serial) -> Result<Self, ShopError> {
        Ok(ItemImageEntity {
            id: object::random_uuid(),
            item_id: serial.item_id.clone(),
            alt_text: serial.alt_text.clone(),
            priority: serial.priority.clone(),
            original_file_name: serial.original_file_name.clone(),
        })
    }

    fn to_entity(&self) -> Self::Entity {
        self.clone()
    }

    fn try_from_entity(entity: &Self::Entity) -> Result<Self, ShopError> {
        Ok(entity.clone())
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ItemImageSerial {
    #[serde(default)]
    pub id: Uuid,
    pub item_id: Uuid,
    pub alt_text: String,
    pub priority: i32,
    pub original_file_name: String,
}

impl ShopSerial for ItemImageSerial {
    type Model = ItemImage;
}

impl JsonHttpResponse for ItemImageSerial {}
impl JsonHttpResponse for Vec<ItemImageSerial> {}
