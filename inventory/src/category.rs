use crate::server::JsonHttpResponse;
use serde::{Deserialize, Serialize};
use sqlx::postgres::PgQueryResult;
use sqlx::{query, query_as, Error, PgPool, Pool, Postgres};
use uuid::Uuid;

#[derive(Debug)]
pub struct Category {
	pub id: Uuid,
	pub display_name: String,
	pub internal_name: String,
	pub parent_id: Option<Uuid>,
}

impl crate::Resource for Category {
	type Serializable = CategorySerial;

	fn to_serial(&self) -> CategorySerial {
		CategorySerial {
			id: self.id.clone(),
			display_name: self.display_name.clone(),
			internal_name: self.internal_name.clone(),
			parent_id: self.parent_id.clone(),
		}
	}

	fn from_serial(serial: &CategorySerial) -> Category {
		Category {
			id: serial.id.clone(),
			display_name: serial.display_name.clone(),
			internal_name: serial.internal_name.clone(),
			parent_id: serial.parent_id.clone(),
		}
	}
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct CategorySerial {
	pub id: Uuid,
	pub display_name: String,
	pub internal_name: String,
	pub parent_id: Option<Uuid>,
}

impl Default for CategorySerial {
	fn default() -> Self {
		CategorySerial {
			id: crate::random_uuid(),
			display_name: String::new(),
			internal_name: String::new(),
			parent_id: None,
		}
	}
}
impl JsonHttpResponse for CategorySerial {}
impl JsonHttpResponse for Vec<CategorySerial> {}

pub async fn get_all_categories(pool: &PgPool) -> Result<Vec<Category>, Error> {
	query_as!(Category, "SELECT * FROM category")
		.fetch_all(pool)
		.await
}

pub async fn get_category(pool: &Pool<Postgres>, id: Uuid) -> Result<Option<Category>, Error> {
	query_as!(Category, "SELECT * FROM category WHERE id = $1", id)
		.fetch_optional(pool)
		.await
}

pub async fn create_category(
	pool: &Pool<Postgres>,
	category: Category,
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

pub mod route {
	use super::*;
	use crate::Resource;
	use actix_web::http::StatusCode;
	use actix_web::{get, post, web, HttpResponseBuilder, Responder};

	pub fn configurer(config: &mut web::ServiceConfig) {
		config.service(
			web::scope("/category")
				.service(get_all_categories)
				.service(get_category)
				.service(create_category),
		);
	}

	#[get("")]
	pub async fn get_all_categories(pgpool: web::Data<PgPool>) -> impl Responder {
		let all_categories = super::get_all_categories(pgpool.get_ref()).await;
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
	pub async fn get_category(
		pgpool: web::Data<Pool<Postgres>>,
		category_id: web::Path<String>,
	) -> impl Responder {
		let Ok(category_id) = Uuid::try_parse(category_id.into_inner().as_str()) else {
			return HttpResponseBuilder::new(StatusCode::BAD_REQUEST).finish();
		};

		let category = super::get_category(pgpool.get_ref(), category_id).await;
		let Ok(category) = category else {
			return HttpResponseBuilder::new(StatusCode::INTERNAL_SERVER_ERROR).finish();
		};
		category
			.map(|category| category.to_serial().to_http_response())
			.unwrap_or(HttpResponseBuilder::new(StatusCode::INTERNAL_SERVER_ERROR).finish())
	}

	#[post("")]
	pub async fn create_category(
		pgpool: web::Data<Pool<Postgres>>,
		body: web::Json<CategorySerial>,
	) -> impl Responder {
		let category = Category::from_serial(&body.into_inner());

		let result = super::create_category(&pgpool, category).await;
		match result {
			Ok(pg_query_result) => HttpResponseBuilder::new(StatusCode::CREATED)
				.body(pg_query_result.rows_affected().to_string()),
			Err(_) => HttpResponseBuilder::new(StatusCode::INTERNAL_SERVER_ERROR).finish(),
		}
	}
}
