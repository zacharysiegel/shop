use crate::InventoryResource;
use crate::category::Category;
use crate::server::JsonHttpResponse;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::postgres::PgQueryResult;
use sqlx::types::chrono;
use sqlx::{Error, PgPool, query, query_as};
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

impl InventoryResource for Product {
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

pub async fn create_product(pgpool: &PgPool, product: Product) -> Result<PgQueryResult, Error> {
	query!(
		"\
		insert into product (id, display_name, internal_name, upc, release_date, created, updated)\
		values ($1, $2, $3, $4, $5, $6, $7)\
		",
		product.id,
		product.display_name,
		product.internal_name,
		product.upc,
		product.release_date,
		product.created,
		product.updated
	)
	.execute(pgpool)
	.await
}

pub async fn create_product_category_association(
	pgpool: &PgPool,
	product_id: Uuid,
	category_id: Uuid,
) -> Result<PgQueryResult, Error> {
	query!(
		"\
		insert into product_category_association (category_id, product_id)\
		values ($1, $2)\
		",
		category_id,
		product_id,
	)
	.execute(pgpool)
	.await
}

pub mod route {
	use super::*;
	use crate::InventoryResource;
	use crate::category::CategorySerial;
	use actix_web::http::StatusCode;
	use actix_web::{HttpResponseBuilder, Responder, get, post, web};

	pub fn configurer(config: &mut web::ServiceConfig) {
		config.service(
			web::scope("/product")
				.service(get_product)
				.service(get_product_categories)
				.service(create_product)
				.service(create_product_category_association),
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

	// todo: restrict to authenticated administrator
	#[post("")]
	pub async fn create_product(
		pgpool: web::Data<PgPool>,
		body: web::Json<ProductSerial>,
	) -> impl Responder {
		let product: Product = Product::from_serial(&body.into_inner());

		let result: Result<PgQueryResult, Error> = super::create_product(&pgpool, product).await;
		match result {
			Ok(query_result) => HttpResponseBuilder::new(StatusCode::CREATED)
				.body(query_result.rows_affected().to_string()),
			Err(_) => HttpResponseBuilder::new(StatusCode::INTERNAL_SERVER_ERROR).finish(),
		}
	}

	// todo: restrict to authenticated administrator
	#[post("/{product_id}/category/{category_id}")]
	pub async fn create_product_category_association(
		pgpool: web::Data<PgPool>,
		path: web::Path<(String, String)>,
	) -> impl Responder {
		let (product_id, category_id) = path.into_inner();
		let Ok(product_id) = Uuid::try_parse(product_id.as_str()) else {
			return HttpResponseBuilder::new(StatusCode::INTERNAL_SERVER_ERROR).finish();
		};
		let Ok(category_id) = Uuid::try_parse(category_id.as_str()) else {
			return HttpResponseBuilder::new(StatusCode::INTERNAL_SERVER_ERROR).finish();
		};

		let result =
			super::create_product_category_association(&pgpool, product_id, category_id).await;
		match result {
			Ok(query_result) => HttpResponseBuilder::new(StatusCode::CREATED)
				.body(query_result.rows_affected().to_string()),
			Err(_) => HttpResponseBuilder::new(StatusCode::INTERNAL_SERVER_ERROR).finish(),
		}
	}
}
