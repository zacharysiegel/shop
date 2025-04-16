mod state;

use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use state::AppState;

pub async fn open_server() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .app_data(web::Data::new(AppState::new()))
            .service(hello)
            .route("/me", web::get().to(me))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

#[get("/hello")]
async fn hello(data: web::Data<AppState>) -> impl Responder {
    HttpResponse::Ok().body(format!("[{}] Hello world!", data.artifact_id))
}

async fn me() -> impl Responder {
    HttpResponse::Ok().body("Zachary Siegel")
}
