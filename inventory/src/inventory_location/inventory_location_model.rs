use crate::error::ShopError;
use crate::object::JsonHttpResponse;
use crate::{object, ShopEntity, ShopModel, ShopSerial};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct InventoryLocationEntity {
    pub id: Uuid,
    pub display_name: String,
    pub internal_name: String,
}

impl ShopEntity for InventoryLocationEntity {
    type Model = InventoryLocation;
}

pub type InventoryLocation = InventoryLocationEntity;

impl ShopModel for InventoryLocation {
    type Entity = Self;
    type Serial = InventoryLocationSerial;

    fn to_serial(&self) -> Self::Serial {
        InventoryLocationSerial {
            id: self.id,
            display_name: self.display_name.clone(),
            internal_name: self.internal_name.clone(),
        }
    }

    fn try_from_serial(serial: &Self::Serial) -> Result<Self, ShopError> {
        Ok(InventoryLocationEntity {
            id: object::random_uuid(),
            display_name: serial.display_name.clone(),
            internal_name: serial.internal_name.clone(),
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
pub struct InventoryLocationSerial {
    #[serde(default)]
    pub id: Uuid,
    pub display_name: String,
    pub internal_name: String,
}

impl ShopSerial for InventoryLocationSerial {
    type Model = InventoryLocation;
}
impl JsonHttpResponse for InventoryLocationSerial {}
impl JsonHttpResponse for Vec<InventoryLocationSerial> {}
