use crate::error::ShopError;
use crate::object::JsonHttpResponse;
use crate::pagination::KeysetPaginationResultForString;
use crate::{object, ShopEntity, ShopModel, ShopSerial};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::types::chrono;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct ProductEntity {
    pub id: Uuid,
    pub display_name: String,
    pub internal_name: String,
    pub upc: Option<String>,
    pub release_date: Option<chrono::NaiveDate>,
    pub created: DateTime<Utc>,
    pub updated: DateTime<Utc>,
}

impl ShopEntity for ProductEntity {
    type Model = Self;
}

pub type Product = ProductEntity; // todo: Refactor other dual entity/model types to use this pattern

impl ShopModel for Product {
    type Entity = Self;
    type Serial = ProductSerial;

    fn to_serial(&self) -> Self::Serial {
        ProductSerial {
            id: self.id.clone(),
            display_name: self.display_name.clone(),
            internal_name: self.internal_name.clone(),
            upc: self.upc.clone(),
            release_date: self.release_date.clone(),
        }
    }

    fn try_from_serial(serial: &Self::Serial) -> Result<Self, ShopError> {
        Ok(ProductEntity {
            id: object::random_uuid(),
            display_name: serial.display_name.clone(),
            internal_name: serial.internal_name.clone(),
            upc: serial.upc.clone(),
            release_date: serial.release_date.clone(),
            created: Utc::now(),
            updated: Utc::now(),
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
pub struct ProductSerial {
    #[serde(default)]
    pub id: Uuid,
    pub display_name: String,
    pub internal_name: String,
    pub upc: Option<String>,
    pub release_date: Option<chrono::NaiveDate>,
}

impl ShopSerial for ProductSerial {
    type Model = ProductEntity;
}
impl JsonHttpResponse for ProductSerial {}
impl JsonHttpResponse for Vec<ProductSerial> {}
impl JsonHttpResponse for (Vec<ProductSerial>, KeysetPaginationResultForString) {}
