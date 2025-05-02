#[macro_export]
macro_rules! unwrap_result_else_markup {
    ($result:expr$(,)?) => {
        match $result {
            ::std::result::Result::Ok(result) => result,
            ::std::result::Result::Err(markup) => return markup,
        }
    };
}