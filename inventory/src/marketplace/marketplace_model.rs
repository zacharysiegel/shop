use crate::error::ShopError;
use crate::object::JsonHttpResponse;
use crate::{object, ShopEntity, ShopModel, ShopSerial};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
#[derive(Debug, Clone)]
pub struct MarketplaceEntity {
    pub id: Uuid,
    pub display_name: String,
    pub internal_name: String,
    pub uri: Option<String>,
}

impl ShopEntity for MarketplaceEntity {
    type Model = MarketplaceEntity;
}
impl ShopModel for MarketplaceEntity {
    type Entity = MarketplaceEntity;
    type Serial = MarketplaceSerial;

    fn to_serial(&self) -> Self::Serial {
        MarketplaceSerial {
            id: self.id.clone(),
            display_name: self.display_name.clone(),
            internal_name: self.internal_name.clone(),
            uri: self.uri.clone(),
        }
    }

    fn try_from_serial(serial: &Self::Serial) -> Result<Self, ShopError> {
        Ok(MarketplaceEntity {
            id: object::random_uuid(),
            display_name: serial.display_name.clone(),
            internal_name: serial.internal_name.clone(),
            uri: serial.uri.clone(),
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
pub struct MarketplaceSerial {
    pub id: Uuid,
    pub display_name: String,
    pub internal_name: String,
    pub uri: Option<String>,
}

impl ShopSerial for MarketplaceSerial {
    type Model = MarketplaceEntity;
}

impl JsonHttpResponse for MarketplaceSerial {}
impl JsonHttpResponse for Vec<MarketplaceSerial> {}
