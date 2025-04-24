use actix_web::{middleware, web, App, HttpResponse, HttpServer};

pub async fn open_server() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .default_service(web::route().to(HttpResponse::NotFound))
            .configure(crate::public::configuration)
            .service(web::scope("/admin").configure(crate::admin::configuration))
            .service(web::scope("/api").configure(crate::api::configuration))
    })
        .bind("127.0.0.1:11000")?
        .run()
        .await
}
