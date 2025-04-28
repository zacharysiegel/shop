use actix_web::guard;
use serde::Deserialize;
use std::borrow::Cow;

pub fn pagination_guard(ctx: &guard::GuardContext) -> bool {
    ctx.head()
        .uri
        .query()
        .unwrap_or("")
        .contains("page_size")
}

#[derive(Debug, Clone, Deserialize)]
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
#[derive(Debug, Deserialize)]
pub struct KeysetPaginationOptionsForStr<'start_value> {
    pub page_size: u32,

    /// If none, returns the first page
    pub start_value: Option<Cow<'start_value, str>>,

    /// If none, a default is used. Default varies per table.
    pub sort_order: Option<SortOrder>,
}
