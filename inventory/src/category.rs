use crate::server::JsonHttpResponse;
use crate::{ShopEntity, ShopModel, ShopSerial};
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, Pool, Postgres};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct CategoryEntity {
	pub id: Uuid,
	pub display_name: String,
	pub internal_name: String,
	pub parent_id: Option<Uuid>,
}

impl ShopEntity for CategoryEntity {
	type Model = Self;
}
impl ShopModel for CategoryEntity {
	type Entity = Self;
	type Serial = CategorySerial;

	fn to_serial(&self) -> CategorySerial {
		CategorySerial {
			id: self.id.clone(),
			display_name: self.display_name.clone(),
			internal_name: self.internal_name.clone(),
			parent_id: self.parent_id.clone(),
		}
	}

	fn from_serial(serial: &CategorySerial) -> CategoryEntity {
		CategoryEntity {
			id: serial.id.clone(),
			display_name: serial.display_name.clone(),
			internal_name: serial.internal_name.clone(),
			parent_id: serial.parent_id.clone(),
		}
	}

	fn to_entity(&self) -> Self::Entity {
		self.clone()
	}

	fn from_entity(entity: &Self::Entity) -> Self {
		entity.clone()
	}
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CategorySerial {
	#[serde(skip_deserializing, default = "crate::random_uuid")]
	pub id: Uuid,
	pub display_name: String,
	pub internal_name: String,
	pub parent_id: Option<Uuid>,
}

impl ShopSerial for CategorySerial {
	type Model = CategoryEntity;
}
impl JsonHttpResponse for CategorySerial {}
impl JsonHttpResponse for Vec<CategorySerial> {}

mod db {
	use crate::category::CategoryEntity;
	use sqlx::postgres::PgQueryResult;
	use sqlx::{Error, PgPool, Pool, Postgres, query, query_as};
	use uuid::Uuid;

	pub async fn get_all_categories(pool: &PgPool) -> Result<Vec<CategoryEntity>, Error> {
		query_as!(CategoryEntity, "SELECT * FROM category")
			.fetch_all(pool)
			.await
	}

	pub async fn get_category(
		pool: &Pool<Postgres>,
		id: Uuid,
	) -> Result<Option<CategoryEntity>, Error> {
		query_as!(CategoryEntity, "SELECT * FROM category WHERE id = $1", id)
			.fetch_optional(pool)
			.await
	}

	// todo: restrict to authenticated administrator
	pub async fn create_category(
		pool: &Pool<Postgres>,
		category: CategoryEntity,
	) -> Result<PgQueryResult, Error> {
		query!(
			"insert into category (id, display_name, internal_name, parent_id) values ($1, $2, $3, $4)",
			category.id,
			category.display_name,
			category.internal_name,
			category.parent_id
		)
		.execute(pool)
		.await
	}
}

pub mod route {
	use super::*;
	use actix_web::http::StatusCode;
	use actix_web::{HttpResponseBuilder, Responder, get, post, web};

	pub fn configurer(config: &mut web::ServiceConfig) {
		config.service(
			web::scope("/category")
				.service(get_all_categories)
				.service(get_category)
				.service(create_category),
		);
	}

	#[get("")]
	async fn get_all_categories(pgpool: web::Data<PgPool>) -> impl Responder {
		let all_categories = db::get_all_categories(pgpool.get_ref()).await;
		let Ok(all_categories) = all_categories else {
			return HttpResponseBuilder::new(StatusCode::INTERNAL_SERVER_ERROR).finish();
		};
		all_categories
			.iter()
			.map(|category| category.to_serial())
			.collect::<Vec<CategorySerial>>()
			.to_http_response()
	}

	#[get("/{category_id}")]
	async fn get_category(
		pgpool: web::Data<Pool<Postgres>>,
		category_id: web::Path<String>,
	) -> impl Responder {
		let Ok(category_id) = Uuid::try_parse(category_id.into_inner().as_str()) else {
			return HttpResponseBuilder::new(StatusCode::BAD_REQUEST).finish();
		};

		let category = db::get_category(pgpool.get_ref(), category_id).await;
		let Ok(category) = category else {
			return HttpResponseBuilder::new(StatusCode::INTERNAL_SERVER_ERROR).finish();
		};
		category
			.map(|category| category.to_serial().to_http_response())
			.unwrap_or(HttpResponseBuilder::new(StatusCode::INTERNAL_SERVER_ERROR).finish())
	}

	#[post("")]
	async fn create_category(
		pgpool: web::Data<Pool<Postgres>>,
		body: web::Json<CategorySerial>,
	) -> impl Responder {
		let category = CategoryEntity::from_serial(&body.into_inner());

		let result = db::create_category(&pgpool, category).await;
		match result {
			Ok(pg_query_result) => HttpResponseBuilder::new(StatusCode::CREATED)
				.body(pg_query_result.rows_affected().to_string()),
			Err(_) => HttpResponseBuilder::new(StatusCode::INTERNAL_SERVER_ERROR).finish(),
		}
	}
}
