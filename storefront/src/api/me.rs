use actix_web::{HttpResponse, Responder};

pub async fn me() -> impl Responder {
	HttpResponse::Ok().body("Zachary Siegel")
}
