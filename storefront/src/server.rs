use actix_web::{middleware, web, App, HttpResponse, HttpServer};
use actix_web_static_files::ResourceFiles;
use static_files::Resource;
use std::collections::HashMap;

pub async fn open_server() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .default_service(web::route().to(HttpResponse::NotFound))
            .configure(crate::www::configurer)
            .configure(crate::admin::index_page::configurer)
            .service(ResourceFiles::new("/", generate_static_file_map()).do_not_resolve_defaults())
    })
        .bind("127.0.0.1:11000")?
        .run()
        .await
}

fn generate_static_file_map() -> HashMap<&'static str, Resource> {
    include!(concat!(env!("OUT_DIR"), "/generated.rs"))
}
