use actix_web::guard;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use crate::error::ShopError;

pub fn pagination_guard(ctx: &guard::GuardContext) -> bool {
    ctx.head()
        .uri
        .query()
        .unwrap_or("")
        .contains("page_size")
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SortOrder {
    #[serde(rename = "asc")]
    Ascending,
    #[serde(rename = "desc")]
    Descending,
}

impl Default for SortOrder {
    fn default() -> Self {
        SortOrder::Ascending
    }
}

/// T: Type of sorted column (as expressed in the ShopEntity implementor)
#[derive(Debug, Serialize, Deserialize)]
pub struct KeysetPaginationOptionsForStr<'start_value> {
    pub page_size: u32,

    /// If none, returns the first page
    pub start_value: Option<Cow<'start_value, str>>,

    /// If none, a default is used. Default varies per table.
    pub sort_order: Option<SortOrder>,
}

impl KeysetPaginationOptionsForStr<'_> {
    pub fn validated(self) -> Result<Self, ShopError> {
        let will_overflow: bool = self.page_size.overflowing_add(1).1;
        match will_overflow {
            true => Err(ShopError { message: format!("Error validating pagination options [{:?}]", self) }),
            false => Ok(self)
        }
    }
}

impl Default for KeysetPaginationOptionsForStr<'_> {
    fn default() -> Self {
        KeysetPaginationOptionsForStr {
            page_size: 50,
            start_value: None,
            sort_order: Some(SortOrder::Ascending),
        }
    }
}
