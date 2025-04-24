#[macro_export]
macro_rules! impl_try_from_custom {
    ($id:ident<$typ:ty>) => {
        impl $id {
            /// Wraps the IntEnum derived try_from implementation to return a result containing ShopError
            pub fn try_from_with_shoperror(v: $typ) -> Result<Self, $crate::error::ShopError> {
                match $id::try_from(v.clone()) {
                    Ok(variant) => Ok(variant),
                    Err(value) => Err($crate::error::ShopError {
                        message: ::std::string::String::from(format!("Error parsing enumeration [{}]", value)),
                    }),
                }
            }
        }
    };
}
