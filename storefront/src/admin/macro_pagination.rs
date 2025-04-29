#[macro_export]
macro_rules! url_encoded_pagination_options_else_err {
    ($options:expr$(,)?) => {
        match ::serde_urlencoded::to_string($options) {
            ::std::result::Result::Ok(val) => val,
            ::std::result::Result::Err(error) => return $crate::admin::structure::error_text::error_text(error),
        }
    };
}
