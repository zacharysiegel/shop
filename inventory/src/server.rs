use actix_web::{middleware, web, App, HttpResponse, HttpServer};
use sqlx::{Pool, Postgres};

pub async fn open_server(pgpool: Pool<Postgres>) -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .wrap(middleware::NormalizePath::trim())
            .app_data(web::Data::new(pgpool.clone()))
            .default_service(web::route().to(HttpResponse::NotFound))
            .configure(crate::public_api::configurer)
            .configure(crate::category::category_api::configurer)
            .configure(crate::product::product_api::configurer)
            .configure(crate::inventory_location::inventory_location_api::configurer)
            .configure(crate::item::item_api::configurer)
            .configure(crate::label::label_api::configurer)
            .configure(crate::item_attribute::item_attribute_api::configurer)
            .configure(crate::metric_counter::metric_counter_api::configurer)
            .configure(crate::customer::customer_api::configurer)
            .configure(crate::marketplace::marketplace_api::configurer)
            .configure(crate::listing::listing_api::configurer)
            .configure(crate::purchase::purchase_api::configurer)
    })
        .bind("127.0.0.1:11001")?
        .run()
        .await
}
