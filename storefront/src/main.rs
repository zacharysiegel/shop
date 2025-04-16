use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .route("/me", web::get().to(me))
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}

#[get("/hello")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

async fn me() -> impl Responder {
    HttpResponse::Ok().body("Zachary Siegel")
}
