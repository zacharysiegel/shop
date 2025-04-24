use actix_web::http::StatusCode;
use actix_web::{App, HttpResponse, HttpResponseBuilder, HttpServer, middleware, web};
use serde::Serialize;
use sqlx::{Pool, Postgres};

pub async fn open_server(pgpool: Pool<Postgres>) -> std::io::Result<()> {
	HttpServer::new(move || {
		App::new()
			.wrap(middleware::Logger::default())
			.app_data(web::Data::new(pgpool.clone()))
			.default_service(web::route().to(HttpResponse::NotFound))
			.configure(crate::category::category_api::configurer)
			.configure(crate::product::product_api::configurer)
			.configure(crate::inventory_location::inventory_location_api::configurer)
			.configure(crate::item::item_api::configurer)
			.configure(crate::label::label_api::configurer)
			.configure(crate::item_image::item_image_api::configurer)
			.configure(crate::item_attribute::item_attribute_api::configurer)
			.configure(crate::metric_counter::metric_counter_api::configurer)
			.configure(crate::customer::customer_api::configurer)
			.configure(crate::marketplace::marketplace_api::configurer)
			.configure(crate::listing::listing_api::configurer)
	})
	.bind("127.0.0.1:11001")?
	.run()
	.await
}

pub trait JsonHttpResponse
where
	Self: Sized + Serialize,
{
	fn to_http_response(&self) -> HttpResponse {
		let Ok(json) = serde_json::to_string(&self) else {
			return HttpResponseBuilder::new(StatusCode::INTERNAL_SERVER_ERROR).body(());
		};
		HttpResponseBuilder::new(StatusCode::OK).body(json)
	}
}
