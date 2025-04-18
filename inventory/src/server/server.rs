use actix_web::{App, HttpResponse, HttpServer, middleware, web};
use sqlx::{Pool, Postgres};

pub async fn open_server(pgpool: Pool<Postgres>) -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .app_data(web::Data::new(pgpool.clone()))
            .default_service(web::route().to(HttpResponse::NotFound))
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
