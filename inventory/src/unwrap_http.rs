#[macro_export]
macro_rules! unwrap_result_else_400 {
    ($e:expr) => {
        match $e {
            Ok(content) => content,
            Err(error) => {
                ::log::error!("Error unwrapping Result; Returning 400 Bad Request; [{:#}];", error.to_string());
                return ::actix_web::HttpResponse::BadRequest().finish();
            },
        }
    };
}

#[macro_export]
macro_rules! unwrap_result_else_500 {
    ($e:expr) => {
        match $e {
            Ok(content) => content,
            Err(error) => {
                ::log::error!("Error unwrapping Result; Returning 500 Internal Server Error; [{:#}];", error.to_string());
                return ::actix_web::HttpResponse::InternalServerError().finish();
            },
        }
    };
}

#[macro_export]
macro_rules! unwrap_option_else_404 {
    ($e:expr) => {
        match $e {
            Some(content) => content,
            None => {
                ::log::error!("Error unwrapping Option; Returning 404 Not Found;");
                return ::actix_web::HttpResponse::NotFound().finish();
            },
        }
    };
}
