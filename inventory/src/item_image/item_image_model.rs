use crate::error::ShopError;
use crate::{ShopEntity, ShopModel, ShopSerial};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::server::JsonHttpResponse;

#[derive(Debug, Clone)]
pub struct ItemImage {
    pub id: Uuid,
    pub item_id: Uuid,
    pub uri: String,
    pub alt_text: String,
    pub priority: i32,
}

impl ShopEntity for ItemImage {
    type Model = ItemImage;
}
impl ShopModel for ItemImage {
    type Entity = ItemImage;
    type Serial = ItemImageSerial;

    fn to_serial(&self) -> Self::Serial {
        ItemImageSerial {
            id: self.id.clone(),
            item_id: self.item_id.clone(),
            uri: self.uri.clone(),
            alt_text: self.alt_text.clone(),
            priority: self.priority.clone(),
        }
    }

    fn try_from_serial(serial: &Self::Serial) -> Result<Self, ShopError> {
        Ok(ItemImage {
            id: serial.id.clone(),
            item_id: serial.item_id.clone(),
            uri: serial.uri.clone(),
            alt_text: serial.alt_text.clone(),
            priority: serial.priority.clone(),
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
    #[serde(skip_deserializing, default = "crate::random_uuid")]
    pub id: Uuid,
    pub item_id: Uuid,
    pub uri: String,
    pub alt_text: String,
    pub priority: i32,
}

impl ShopSerial for ItemImageSerial {
    type Model = ItemImage;
}

impl JsonHttpResponse for ItemImageSerial {}
impl JsonHttpResponse for Vec<ItemImageSerial> {}
