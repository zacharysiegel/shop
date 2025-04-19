use crate::InventoryEntity;
use crate::server::JsonHttpResponse;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug)]
pub struct InventoryLocationEntity {
	pub id: Uuid,
	pub display_name: String,
	pub internal_name: String,
}

impl InventoryEntity for InventoryLocationEntity {
	type Serializable = InventoryLocationSerial;

	fn to_serial(&self) -> Self::Serializable {
		InventoryLocationSerial {
			id: self.id,
			display_name: self.display_name.clone(),
			internal_name: self.internal_name.clone(),
		}
	}

	fn from_serial(serializable: &Self::Serializable) -> Self {
		InventoryLocationEntity {
			id: serializable.id.clone(),
			display_name: serializable.display_name.clone(),
			internal_name: serializable.internal_name.clone(),
		}
	}
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InventoryLocationSerial {
	#[serde(skip_deserializing, default = "crate::random_uuid")]
	pub id: Uuid,
	pub display_name: String,
	pub internal_name: String,
}

impl JsonHttpResponse for InventoryLocationSerial {}
impl JsonHttpResponse for Vec<InventoryLocationSerial> {}

pub mod db {
	use crate::inventory_location::InventoryLocationEntity;
	use sqlx::postgres::PgQueryResult;
	use sqlx::{Error, PgPool, query, query_as};

	pub async fn create_inventory_location(
		pgpool: &PgPool,
		inventory_location: InventoryLocationEntity,
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

	pub async fn get_all_inventory_locations(
		pgpool: &PgPool,
	) -> Result<Vec<InventoryLocationEntity>, Error> {
		query_as!(
			InventoryLocationEntity,
			"select id, display_name, internal_name from inventory_location"
		)
		.fetch_all(pgpool)
		.await
	}
}

pub mod route {
	use super::*;
	use actix_web::http::StatusCode;
	use actix_web::{HttpResponseBuilder, Responder, get, post, web};
	use sqlx::{Pool, Postgres};

	pub fn configurer(config: &mut web::ServiceConfig) {
		config.service(
			web::scope("/inventory_location")
				.service(create_inventory_location)
				.service(get_all_inventory_locations),
		);
	}

	#[post("")]
	async fn create_inventory_location(
		pgpool: web::Data<Pool<Postgres>>,
		body: web::Json<InventoryLocationSerial>,
	) -> impl Responder {
		let inventory_location = body.into_inner();
		let inventory_location = InventoryLocationEntity::from_serial(&inventory_location);

		let result = db::create_inventory_location(&pgpool, inventory_location).await;
		match result {
			Ok(query_result) => HttpResponseBuilder::new(StatusCode::CREATED)
				.body(query_result.rows_affected().to_string()),
			Err(_) => HttpResponseBuilder::new(StatusCode::INTERNAL_SERVER_ERROR).finish(),
		}
	}

	#[get("")]
	async fn get_all_inventory_locations(pgpool: web::Data<Pool<Postgres>>) -> impl Responder {
		let result = db::get_all_inventory_locations(&pgpool).await;
		let inventory_locations = match result {
			Ok(inventory_locations) => inventory_locations,
			Err(e) => {
				log::error!("Error fetching inventory locations; [{:#}];", e);
				return HttpResponseBuilder::new(StatusCode::INTERNAL_SERVER_ERROR).finish();
			}
		};

		inventory_locations
			.iter()
			.map(|inventory_location| InventoryLocationEntity::to_serial(inventory_location))
			.collect::<Vec<InventoryLocationSerial>>()
			.to_http_response()
	}
}
