use crate::InventoryResource;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug)]
pub struct InventoryLocation {
	pub id: Uuid,
	pub display_name: String,
	pub internal_name: String,
}

impl InventoryResource for InventoryLocation {
	type Serializable = InventoryLocationSerial;

	fn to_serial(&self) -> Self::Serializable {
		InventoryLocationSerial {
			id: self.id,
			display_name: self.display_name.clone(),
			internal_name: self.internal_name.clone(),
		}
	}

	fn from_serial(serializable: &Self::Serializable) -> Self {
		InventoryLocation {
			id: serializable.id.clone(),
			display_name: serializable.display_name.clone(),
			internal_name: serializable.internal_name.clone(),
		}
	}
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(default)] // todo: this mechanism allows clients to choose their own ids
pub struct InventoryLocationSerial {
	pub id: Uuid,
	pub display_name: String,
	pub internal_name: String,
}

impl Default for InventoryLocationSerial {
	fn default() -> Self {
		InventoryLocationSerial {
			id: crate::random_uuid(),
			display_name: String::new(),
			internal_name: String::new(),
		}
	}
}

pub mod db {
	use crate::inventory_location::InventoryLocation;
	use sqlx::postgres::PgQueryResult;
	use sqlx::{Error, PgPool, query};

	pub async fn create_inventory_location(
		pgpool: &PgPool,
		inventory_location: InventoryLocation,
	) -> Result<PgQueryResult, Error> {
		query!(
			"\
			insert into inventory_location (id, display_name, internal_name) \
			values ($1, $2, $3)\
			",
			inventory_location.id,
			inventory_location.display_name,
			inventory_location.internal_name
		)
		.execute(pgpool)
		.await
	}
}

pub mod route {
	use super::*;
	use actix_web::http::StatusCode;
	use actix_web::{HttpResponseBuilder, Responder, post, web};
	use sqlx::{Pool, Postgres};

	pub fn configurer(config: &mut web::ServiceConfig) {
		config.service(web::scope("/inventory_location").service(create_inventory_location));
	}

	#[post("")]
	async fn create_inventory_location(
		pgpool: web::Data<Pool<Postgres>>,
		body: web::Json<InventoryLocationSerial>,
	) -> impl Responder {
		let inventory_location = body.into_inner();
		let inventory_location = InventoryLocation::from_serial(&inventory_location);

		let result = super::db::create_inventory_location(&pgpool, inventory_location).await;
		match result {
			Ok(query_result) => HttpResponseBuilder::new(StatusCode::CREATED)
				.body(query_result.rows_affected().to_string()),
			Err(_) => HttpResponseBuilder::new(StatusCode::INTERNAL_SERVER_ERROR).finish(),
		}
	}
}
