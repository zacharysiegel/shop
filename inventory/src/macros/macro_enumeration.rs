/// Requires #[derive(strum::FromRepr)] on the enum
#[macro_export]
macro_rules! try_from_repr {
	($id:ident<$typ:ty>$(,)?) => {
		impl $id {
			/// Wraps the strum::FromRepr produced from_repr method to produce a Result<EnumT, ShopError>
			pub fn try_from_repr(
				v: $typ,
			) -> ::std::result::Result<Self, $crate::error::ShopError> {
				match $id::from_repr(v.clone()) {
					::std::option::Option::Some(variant) => ::std::result::Result::Ok(variant),
					::std::option::Option::None => {
						::std::result::Result::Err($crate::error::ShopError {
							message: ::std::string::String::from(format!(
								"Error parsing enumeration [{}]",
								v,
							)),
						})
					}
				}
			}
		}
	};
}
