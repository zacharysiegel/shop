use super::*;
use crate::ShopModel;
use crate::server::JsonHttpResponse;
use actix_web::http::StatusCode;
use actix_web::{HttpResponseBuilder, Responder, get, post, web};
use sqlx::{PgPool, Pool, Postgres};
use uuid::Uuid;

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
	let all_categories = category_db::get_all_categories(pgpool.get_ref()).await;
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
async fn get_category(pgpool: web::Data<PgPool>, category_id: web::Path<String>) -> impl Responder {
	let Ok(category_id) = Uuid::try_parse(category_id.into_inner().as_str()) else {
		return HttpResponseBuilder::new(StatusCode::BAD_REQUEST).finish();
	};

	let category = category_db::get_category(pgpool.get_ref(), category_id).await;
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
	let Ok(category) = CategoryEntity::try_from_serial(&body.into_inner()) else {
		return HttpResponseBuilder::new(StatusCode::BAD_REQUEST).finish();
	};

	let result = category_db::create_category(&pgpool, category).await;
	match result {
		Ok(pg_query_result) => HttpResponseBuilder::new(StatusCode::CREATED)
			.body(pg_query_result.rows_affected().to_string()),
		Err(_) => HttpResponseBuilder::new(StatusCode::INTERNAL_SERVER_ERROR).finish(),
	}
}
