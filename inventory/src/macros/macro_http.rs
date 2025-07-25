#[macro_export]
macro_rules! unwrap_result_else_400 {
	($e:expr) => {
		match $e {
			::std::result::Result::Ok(content) => content,
			::std::result::Result::Err(error) => {
				::log::error!(
					"Error unwrapping Result; Returning 400 Bad Request; [{:#}];",
					error.to_string()
				);
				return ::actix_web::HttpResponse::BadRequest().finish();
			}
		}
	};
}

#[macro_export]
macro_rules! unwrap_result_else_500 {
	($e:expr) => {
		match $e {
			::std::result::Result::Ok(content) => content,
			::std::result::Result::Err(error) => {
				::log::error!(
					"Error unwrapping Result; Returning 500 Internal Server Error; [{:#}];",
					error.to_string()
				);
				return ::actix_web::HttpResponse::InternalServerError().finish();
			}
		}
	};
}

#[macro_export]
macro_rules! unwrap_option_else_404 {
	($e:expr) => {
		match $e {
			::std::option::Option::Some(content) => content,
			::std::option::Option::None => {
				::log::error!("Error unwrapping Option; Returning 404 Not Found;");
				return ::actix_web::HttpResponse::NotFound().finish();
			}
		}
	};
}

#[macro_export]
macro_rules! unwrap_option_else_400 {
	($e:expr) => {
		match $e {
			::std::option::Option::Some(content) => content,
			::std::option::Option::None => {
				::log::error!("Error unwrapping Option; Returning 400 Bad Request;");
				return ::actix_web::HttpResponse::BadRequest().finish();
			}
		}
	};
}
