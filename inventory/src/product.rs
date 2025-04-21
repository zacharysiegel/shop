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

mod db {
	use crate::category::CategoryEntity;
	use crate::product::ProductEntity;
	use sqlx::postgres::PgQueryResult;
	use sqlx::{Error, PgPool, query, query_as};
	use uuid::Uuid;

	pub async fn get_product(
		pgpool: &PgPool,
		product_id: Uuid,
	) -> Result<Option<ProductEntity>, Error> {
		query_as!(
			ProductEntity,
			"select * from product where id = $1",
			product_id
		)
		.fetch_optional(pgpool)
		.await
	}

	pub async fn get_product_categories(
		pgpool: &PgPool,
		product_id: Uuid,
	) -> Result<Vec<CategoryEntity>, Error> {
		query_as!(CategoryEntity, "
        select category.*
		from category
        inner join product_category_association on category.id = product_category_association.category_id
        where product_category_association.product_id = $1
    ", product_id)
			.fetch_all(pgpool)
			.await
	}

	pub async fn create_product(
		pgpool: &PgPool,
		product: ProductEntity,
	) -> Result<PgQueryResult, Error> {
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
}

pub mod route {
	use super::*;
	use crate::ShopModel;
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
	async fn get_product(
		pgpool: web::Data<PgPool>,
		product_id: web::Path<String>,
	) -> impl Responder {
		let Ok(product_id) = Uuid::try_parse(product_id.into_inner().as_str()) else {
			return HttpResponseBuilder::new(StatusCode::INTERNAL_SERVER_ERROR).finish();
		};

		let Ok(product) = db::get_product(&pgpool, product_id).await else {
			return HttpResponseBuilder::new(StatusCode::INTERNAL_SERVER_ERROR).finish();
		};

		product
			.map(|product| product.to_serial().to_http_response())
			.unwrap_or(HttpResponseBuilder::new(StatusCode::INTERNAL_SERVER_ERROR).finish())
	}

	#[get("/{product_id}/category")]
	async fn get_product_categories(
		pgpool: web::Data<PgPool>,
		product_id: web::Path<String>,
	) -> impl Responder {
		let Ok(product_id) = Uuid::try_parse(product_id.into_inner().as_str()) else {
			return HttpResponseBuilder::new(StatusCode::INTERNAL_SERVER_ERROR).finish();
		};

		let Ok(product_categories) = db::get_product_categories(&pgpool, product_id).await else {
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
	async fn create_product(
		pgpool: web::Data<PgPool>,
		body: web::Json<ProductSerial>,
	) -> impl Responder {
		let Ok(product) = ProductEntity::try_from_serial(&body.into_inner()) else {
			return HttpResponseBuilder::new(StatusCode::BAD_REQUEST).finish();
		};

		let result: Result<PgQueryResult, Error> = db::create_product(&pgpool, product).await;
		match result {
			Ok(query_result) => HttpResponseBuilder::new(StatusCode::CREATED)
				.body(query_result.rows_affected().to_string()),
			Err(_) => HttpResponseBuilder::new(StatusCode::INTERNAL_SERVER_ERROR).finish(),
		}
	}

	// todo: restrict to authenticated administrator
	#[post("/{product_id}/category/{category_id}")]
	async fn create_product_category_association(
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
			db::create_product_category_association(&pgpool, product_id, category_id).await;
		match result {
			Ok(query_result) => HttpResponseBuilder::new(StatusCode::CREATED)
				.body(query_result.rows_affected().to_string()),
			Err(_) => HttpResponseBuilder::new(StatusCode::INTERNAL_SERVER_ERROR).finish(),
		}
	}
}
