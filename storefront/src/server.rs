mod route;
mod state;

use actix_web::{web, App, HttpServer};
use state::AppState;

pub async fn open_server() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .app_data(web::Data::new(AppState::new()))
            .configure(route::conf)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
