use crate::category::Category;
use crate::server::JsonHttpResponse;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::types::chrono;
use sqlx::{Error, PgPool, query_as};
use uuid::Uuid;

#[derive(Debug)]
pub struct Product {
	pub id: Uuid,
	pub display_name: String,
	pub internal_name: String,
	pub upc: Option<String>,
	pub release_date: Option<chrono::NaiveDate>,
	pub created: DateTime<Utc>,
	pub updated: DateTime<Utc>,
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
		}
	}

	fn from_serial(serializable: &Self::Serializable) -> Self {
		Product {
			id: serializable.id.clone(),
			display_name: serializable.display_name.clone(),
			internal_name: serializable.internal_name.clone(),
			upc: serializable.upc.clone(),
			release_date: serializable.release_date.clone(),
			created: Utc::now(),
			updated: Utc::now(),
		}
	}
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct ProductSerial {
	pub id: Uuid,
	pub display_name: String,
	pub internal_name: String,
	pub upc: Option<String>,
	pub release_date: Option<chrono::NaiveDate>,
}

impl Default for ProductSerial {
	fn default() -> Self {
		ProductSerial {
			id: crate::random_uuid(),
			display_name: String::new(),
			internal_name: String::new(),
			upc: None,
			release_date: None,
		}
	}
}

impl JsonHttpResponse for ProductSerial {}

pub async fn get_product(pgpool: &PgPool, product_id: Uuid) -> Result<Option<Product>, Error> {
	query_as!(Product, "select * from product where id = $1", product_id)
		.fetch_optional(pgpool)
		.await
}

pub async fn get_product_categories(
	pgpool: &PgPool,
	product_id: Uuid,
) -> Result<Vec<Category>, Error> {
	query_as!(Category, "
        select category.*
		from category
        inner join product_category_association on category.id = product_category_association.category_id
        where product_category_association.product_id = $1
    ", product_id)
        .fetch_all(pgpool)
        .await
}

pub mod route {
	use super::*;
	use crate::Resource;
	use crate::category::CategorySerial;
	use actix_web::http::StatusCode;
	use actix_web::{HttpResponseBuilder, Responder, get, post, web};

	pub fn configurer(config: &mut web::ServiceConfig) {
		config.service(
			web::scope("/product")
				.service(get_product)
				.service(get_product_categories),
		);
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

	#[get("/{product_id}/category")]
	pub async fn get_product_categories(
		pgpool: web::Data<PgPool>,
		product_id: web::Path<String>,
	) -> impl Responder {
		let Ok(product_id) = Uuid::try_parse(product_id.into_inner().as_str()) else {
			return HttpResponseBuilder::new(StatusCode::INTERNAL_SERVER_ERROR).finish();
		};

		let Ok(product_categories) = super::get_product_categories(&pgpool, product_id).await
		else {
			return HttpResponseBuilder::new(StatusCode::INTERNAL_SERVER_ERROR).finish();
		};

		product_categories
			.iter()
			.map(|category| category.to_serial())
			.collect::<Vec<CategorySerial>>()
			.to_http_response()
	}

	// #[post("")]
	// pub async fn create_product(
	// 	pgpool: web::Data<PgPool>,
	// 	body: web::Json<ProductSerial>,
	// ) -> impl Responder {
	// }
}
