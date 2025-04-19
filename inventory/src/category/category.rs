use crate::server::JsonHttpResponse;
use actix_web::http::StatusCode;
use actix_web::{get, web, HttpResponseBuilder, Responder};
use serde::Serialize;
use sqlx::types::Uuid;
use sqlx::{query_as, PgPool, Pool, Postgres};

#[derive(Debug)]
pub struct Category {
	id: Uuid,
	display_name: String,
	internal_name: String,
	parent_id: Option<Uuid>,
}

impl Category {
	pub fn to_response(&self) -> CategoryResponse {
		CategoryResponse {
			id: self.id.to_string(),
			display_name: self.display_name.clone(),
			internal_name: self.internal_name.clone(),
			parent_id: self.parent_id.map(|id| id.to_string()),
		}
	}
}

#[derive(Debug, Serialize)]
pub struct CategoryResponse {
	id: String,
	display_name: String,
	internal_name: String,
	parent_id: Option<String>,
}

impl JsonHttpResponse for CategoryResponse {}
impl JsonHttpResponse for Vec<CategoryResponse> {}

pub async fn get_all_categories(pool: &PgPool) -> Vec<Category> {
	query_as!(Category, "SELECT * FROM category")
		.fetch_all(pool)
		.await
		.unwrap()
}

pub async fn get_category(pool: &Pool<Postgres>, id: Uuid) -> Option<Category> {
	query_as!(Category, "SELECT * FROM category WHERE id = $1", id)
		.fetch_optional(pool)
		.await
		.unwrap()
}

pub mod route {
	use super::*;

	pub fn configurer(config: &mut web::ServiceConfig) {
		config.service(
			web::scope("/category")
				.service(get_all_categories)
				.service(get_category),
		);
	}

	#[get("/")]
	pub async fn get_all_categories(pgpool: web::Data<PgPool>) -> impl Responder {
		super::get_all_categories(&pgpool)
			.await
			.iter()
			.map(|category| category.to_response())
			.collect::<Vec<CategoryResponse>>()
			.to_http_response()
	}

	#[get("/{category_id}")]
	pub async fn get_category(
		state: web::Data<Pool<Postgres>>,
		category_id: web::Path<String>,
	) -> impl Responder {
		let Ok(category_id) = Uuid::try_parse(category_id.into_inner().as_str()) else {
			return HttpResponseBuilder::new(StatusCode::BAD_REQUEST).finish();
		};

		super::get_category(state.get_ref(), category_id)
			.await
			.map(|category| category.to_response().to_http_response())
			.unwrap_or_else(|| HttpResponseBuilder::new(StatusCode::INTERNAL_SERVER_ERROR).finish())
	}
}
