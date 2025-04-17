mod route;
mod state;

use actix_web::{middleware, web, App, HttpResponse, HttpServer};
use state::AppState;

pub async fn open_server() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .app_data(web::Data::new(AppState::new()))
            .wrap(middleware::Logger::default())
            .default_service(web::route().to(|| HttpResponse::NotFound()))
            .configure(route::configuration)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
