#[macro_export]
macro_rules! enum_try_from_int_with_shoperror {
	($id:ident<$typ:ty>) => {
		impl $id {
			/// Wraps the IntEnum derived try_from implementation to return a result containing ShopError
			pub fn try_from_int_with_shoperror(
				v: $typ,
			) -> ::std::result::Result<Self, $crate::error::ShopError> {
				match $id::try_from(v.clone()) {
					::std::result::Result::Ok(variant) => ::std::result::Result::Ok(variant),
					::std::result::Result::Err(value) => {
						::std::result::Result::Err($crate::error::ShopError {
							message: ::std::string::String::from(format!(
								"Error parsing enumeration [{}]",
								value
							)),
						})
					}
				}
			}
		}
	};
}
