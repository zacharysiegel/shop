/// Requires #[derive(strum::FromRepr)] on the enum
#[macro_export]
macro_rules! try_from_repr {
	($id:ident<$typ:ty>$(,)?) => {
		impl $id {
			/// Wraps the strum::FromRepr produced from_repr method to produce a Result<EnumT, ShopError>
			pub fn try_from_repr(
				descriminant: $typ,
			) -> ::std::result::Result<Self, $crate::error::ShopError> {
				match $id::from_repr(descriminant.clone()) {
					::std::option::Option::Some(variant) => ::std::result::Result::Ok(variant),
					::std::option::Option::None => {
						::std::result::Result::Err($crate::error::ShopError::new(&format!(
							"Error parsing enumeration [{}]",
							descriminant,
						)))
					},
				}
			}
		}
	};
}

#[macro_export]
macro_rules! create_json_spec {
	($id:ident<$typ:ty>$(,)?) => {
		impl $id {
			/// Produces a JSON-formatted string specifying the contents of the given enum.
			/// Requires a EnumT::to_serial implementation which returns a String per variant (to use as JSON object key).
			/// Requires strum::FromRepr and strum::VariantArray derived traits.
			/// Requires the serde_json crate.
			pub fn get_json_spec() -> ::std::string::String {
    		    let variant_pairs: ::std::vec::Vec<(&'static str, $typ)> = Self::VARIANTS
    		        .iter()
    		        .map(|variant| (variant.to_serial(), variant.clone() as $typ))
    		        .collect();
    		    let mut json_map = ::serde_json::Map::with_capacity(Self::VARIANTS.len());
    		    for pair in variant_pairs {
    		        json_map.insert(pair.0.to_string(), ::serde_json::json!(pair.1));
    		    }
    		    ::serde_json::to_string(&json_map)
    		        .expect("All keys are strings and values are integers.")
    		}
		}
	}
}
