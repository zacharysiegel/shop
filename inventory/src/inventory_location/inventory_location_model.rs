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
    pub time_zone_id: String,
    pub street_address: String,
    pub municipality: String,
    pub district: String,
    pub postal_area: String,
    pub country: String,
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
            time_zone_id: self.time_zone_id.clone(),
            street_address: self.street_address.clone(),
            municipality: self.municipality.clone(),
            district: self.district.clone(),
            postal_area: self.postal_area.clone(),
            country: self.country.clone(),
        }
    }

    fn try_from_serial(serial: &Self::Serial) -> Result<Self, ShopError> {
        Ok(InventoryLocationEntity {
            id: object::random_uuid(),
            display_name: serial.display_name.clone(),
            internal_name: serial.internal_name.clone(),
            time_zone_id: serial.time_zone_id.clone(),
            street_address: serial.street_address.clone(),
            municipality: serial.municipality.clone(),
            district: serial.district.clone(),
            postal_area: serial.postal_area.clone(),
            country: serial.country.clone(),
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
    pub time_zone_id: String,
    pub street_address: String,
    pub municipality: String,
    pub district: String,
    pub postal_area: String,
    pub country: String,
}

impl ShopSerial for InventoryLocationSerial {
    type Model = InventoryLocation;
}
impl JsonHttpResponse for InventoryLocationSerial {}
impl JsonHttpResponse for Vec<InventoryLocationSerial> {}
