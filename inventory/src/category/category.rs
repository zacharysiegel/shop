use actix_web::body::BoxBody;
use actix_web::http::StatusCode;
use actix_web::{get, web, HttpRequest, HttpResponse, Responder};
use serde::Serialize;
use sqlx::types::Uuid;
use sqlx::{query_as, Pool, Postgres};

#[derive(Debug)]
pub struct Category {
	id: Uuid,
	display_name: String,
	internal_name: String,
	parent_id: Option<Uuid>,
}

#[derive(Debug, Serialize)]
pub struct CategoryResponse {
	id: String,
	display_name: String,
	internal_name: String,
	parent_id: Option<String>,
}

impl From<Category> for CategoryResponse {
	fn from(value: Category) -> Self {
		CategoryResponse {
			id: value.id.to_string(),
			display_name: value.display_name,
			internal_name: value.internal_name,
			parent_id: value.parent_id.map(|id| id.to_string()),
		}
	}
}

impl Responder for CategoryResponse {
	type Body = BoxBody;

	fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
		let Ok(json) = serde_json::to_string(&self) else {
			return HttpResponse::new(StatusCode::INTERNAL_SERVER_ERROR).set_body(BoxBody::new(()));
		};
		HttpResponse::new(StatusCode::OK).set_body(BoxBody::new(json))
	}
}

pub async fn get_category(pool: &Pool<Postgres>, id: Uuid) -> Option<Category> {
	query_as!(Category, "SELECT * FROM category WHERE id = $1", id)
		.fetch_optional(pool)
		.await
		.unwrap()
}

#[get("/category/{category_id}")]
pub async fn get_category_route(
	state: web::Data<Pool<Postgres>>,
	category_id: web::Path<String>,
) -> Option<CategoryResponse> {
	let Ok(category_id) = Uuid::try_parse(category_id.into_inner().as_str()) else {
		return None;
	};

	get_category(state.get_ref(), category_id)
		.await
		.map(|category| category.into())
}
