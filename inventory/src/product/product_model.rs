use crate::error::ShopError;
use crate::server::JsonHttpResponse;
use crate::{ShopEntity, ShopModel, ShopSerial};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::postgres::PgQueryResult;
use sqlx::types::chrono;
use sqlx::{Error, PgPool};
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
impl ShopModel for ProductEntity {
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
			id: serial.id.clone(),
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
	#[serde(skip_deserializing, default = "crate::random_uuid")]
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
