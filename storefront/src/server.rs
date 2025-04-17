mod route;
mod state;

use actix_web::{App, HttpServer, web};
use state::AppState;

pub async fn open_server() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .app_data(web::Data::new(AppState::new()))
            .wrap(actix_web::middleware::Logger::default())
            .configure(route::configuration)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
