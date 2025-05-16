use crate::error::ShopError;
use crate::object::JsonHttpResponse;
use crate::{ShopEntity, ShopModel, ShopSerial};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct ItemAttributeEntity {
    pub item_id: Uuid,
    pub key: String,
    pub value: String,
    pub visible: bool,
    pub priority: i32,
    // constraint pk_item_attribute primary key (item_id, key)
}

impl ShopEntity for ItemAttributeEntity {
    type Model = ItemAttribute;
}

pub type ItemAttribute = ItemAttributeEntity;

impl ShopModel for ItemAttribute {
    type Entity = ItemAttributeEntity;
    type Serial = ItemAttributeSerial;

    fn to_serial(&self) -> Self::Serial {
        ItemAttributeSerial {
            item_id: self.item_id.clone(),
            key: self.key.clone(),
            value: self.value.clone(),
            visible: self.visible.clone(),
            priority: self.priority.clone(),
        }
    }

    fn try_from_serial(serial: &Self::Serial) -> Result<Self, ShopError> {
        Ok(ItemAttributeEntity {
            item_id: serial.item_id.clone(),
            key: serial.key.clone(),
            value: serial.value.clone(),
            visible: serial.visible.clone(),
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
pub struct ItemAttributeSerial {
    pub item_id: Uuid,
    pub key: String,
    pub value: String,
    pub visible: bool,
    pub priority: i32,
}

impl ShopSerial for ItemAttributeSerial {
    type Model = ItemAttribute;
}

impl JsonHttpResponse for ItemAttributeSerial {}
impl JsonHttpResponse for Vec<ItemAttributeSerial> {}
