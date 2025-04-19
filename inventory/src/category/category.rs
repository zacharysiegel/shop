use crate::server::JsonHttpResponse;
use serde::{Deserialize, Serialize};
use sqlx::postgres::PgQueryResult;
use sqlx::types::{Uuid, uuid};
use sqlx::{Error, PgPool, Pool, Postgres, query, query_as};

#[derive(Debug)]
pub struct Category {
	id: Uuid,
	display_name: String,
	internal_name: String,
	parent_id: Option<Uuid>,
}

impl Category {
	pub fn to_serial(&self) -> CategorySerial {
		CategorySerial {
			id: self.id.to_string(),
			display_name: self.display_name.clone(),
			internal_name: self.internal_name.clone(),
			parent_id: self.parent_id.map(|id| id.to_string()),
		}
	}

	pub fn from_serial(serial: &CategorySerial) -> Result<Category, uuid::Error> {
		Ok(Category {
			id: Uuid::parse_str(&serial.id)?,
			display_name: serial.display_name.clone(),
			internal_name: serial.internal_name.clone(),
			parent_id: match serial.parent_id.clone() {
				Some(id) => Some(Uuid::parse_str(id.as_ref())?),
				None => None,
			},
		})
	}
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct CategorySerial {
	id: String,
	display_name: String,
	internal_name: String,
	parent_id: Option<String>,
}

impl Default for CategorySerial {
	fn default() -> Self {
		CategorySerial {
			id: crate::random_uuid().to_string(),
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
			.unwrap_or_else(|| HttpResponseBuilder::new(StatusCode::INTERNAL_SERVER_ERROR).finish())
	}

	#[post("")]
	pub async fn create_category(
		pgpool: web::Data<Pool<Postgres>>,
		body: web::Json<CategorySerial>,
	) -> impl Responder {
		let Ok(category) = Category::from_serial(&body.into_inner()) else {
			return HttpResponseBuilder::new(StatusCode::BAD_REQUEST).finish();
		};

		let result = super::create_category(&pgpool, category).await;
		match result {
			Ok(pg_query_result) => HttpResponseBuilder::new(StatusCode::CREATED)
				.body(pg_query_result.rows_affected().to_string()),
			Err(_) => HttpResponseBuilder::new(StatusCode::INTERNAL_SERVER_ERROR).finish(),
		}
	}
}
