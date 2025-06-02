#[macro_export]
macro_rules! try_return {
    ($e:expr$(,)?) => {
		match $e {
			::std::result::Result::Ok(value) => value,
			::std::result::Result::Err(error) => return error,
		}
	}
}
