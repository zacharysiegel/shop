use crate::server::JsonHttpResponse;
use serde::{Deserialize, Serialize};
use sqlx::types::{Uuid, chrono};
use sqlx::{Error, PgPool, query_as};

#[derive(Debug)]
pub struct Product {
	id: Uuid,
	display_name: String,
	internal_name: String,
	upc: Option<String>,
	release_date: Option<chrono::NaiveDate>,
	created: chrono::NaiveDateTime,
	updated: chrono::NaiveDateTime,
}

impl crate::Resource for Product {
	type Serializable = ProductSerial;

	fn to_serial(&self) -> Self::Serializable {
		ProductSerial {
			id: self.id.clone(),
			display_name: self.display_name.clone(),
			internal_name: self.internal_name.clone(),
			upc: self.upc.clone(),
			release_date: self.release_date.clone(),
			created: self.created.clone(),
			updated: self.updated.clone(),
		}
	}

	fn from_serial(serializable: &Self::Serializable) -> Self {
		Product {
			id: serializable.id.clone(),
			display_name: serializable.display_name.clone(),
			internal_name: serializable.internal_name.clone(),
			upc: serializable.upc.clone(),
			release_date: serializable.release_date.clone(),
			created: serializable.created.clone(),
			updated: serializable.updated.clone(),
		}
	}
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProductSerial {
	id: Uuid,
	display_name: String,
	internal_name: String,
	upc: Option<String>,
	release_date: Option<chrono::NaiveDate>,
	created: chrono::NaiveDateTime,
	updated: chrono::NaiveDateTime,
}

impl JsonHttpResponse for ProductSerial {}

pub async fn get_product(pgpool: &PgPool, product_id: Uuid) -> Result<Option<Product>, Error> {
	query_as!(Product, "select * from product where id = $1", product_id)
		.fetch_optional(pgpool)
		.await
}

pub mod route {
	use super::*;
	use crate::Resource;
	use actix_web::http::StatusCode;
	use actix_web::{HttpResponseBuilder, Responder, get, web};

	pub fn configurer(config: &mut web::ServiceConfig) {
		config.service(web::scope("/product").service(get_product));
	}

	#[get("/{product_id}")]
	pub async fn get_product(
		pgpool: web::Data<PgPool>,
		product_id: web::Path<String>,
	) -> impl Responder {
		let Ok(product_id) = Uuid::try_parse(product_id.into_inner().as_str()) else {
			return HttpResponseBuilder::new(StatusCode::INTERNAL_SERVER_ERROR).finish();
		};

		let Ok(product) = super::get_product(&pgpool, product_id).await else {
			return HttpResponseBuilder::new(StatusCode::INTERNAL_SERVER_ERROR).finish();
		};

		product
			.map(|product| product.to_serial().to_http_response())
			.unwrap_or(HttpResponseBuilder::new(StatusCode::INTERNAL_SERVER_ERROR).finish())
	}
}
