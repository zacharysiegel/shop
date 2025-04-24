use actix_web::{middleware, web, App, HttpResponse, HttpServer};

pub async fn open_server() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .default_service(web::route().to(HttpResponse::NotFound))
            .configure(crate::www::configuration)
            .configure(crate::admin::configuration)
    })
        .bind("127.0.0.1:11000")?
        .run()
        .await
}
