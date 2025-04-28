use crate::error::ShopError;
use actix_web::guard;
use serde::{Deserialize, Serialize};

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
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeysetPaginationOptionsForString {
    /// Maximum number of elements in a returned page.
    /// Maximum allowed value is u32::MAX - 1
    pub page_size: u32,

    /// If none, returns the first page
    pub start_value: Option<String>,

    /// If none, a default is used. Default varies per table.
    pub sort_order: SortOrder,
}

impl KeysetPaginationOptionsForString {
    pub fn validated(self) -> Result<Self, ShopError> {
        let will_overflow: bool = self.page_size.overflowing_add(1).1;
        match will_overflow {
            true => Err(ShopError { message: format!("Error validating pagination options [{:?}]", self) }),
            false => Ok(self)
        }
    }
}

impl Default for KeysetPaginationOptionsForString {
    fn default() -> Self {
        KeysetPaginationOptionsForString {
            page_size: 50,
            start_value: None,
            sort_order: SortOrder::Ascending,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KeysetPaginationResultForString {
    pub page_size: u32,
    /// The first element in the returned page.
    pub start_value: Option<String>,
    /// The next element which would be returned after the end of the returned page.
    pub next_value: Option<String>,
    /// The maximum value for the entire table.
    pub max_value: Option<String>,
    /// The minimum value for the entire table.
    pub min_value: Option<String>,
    /// True iff min_value is less than the minimum value in the current page.
    /// This comparison should be performed by the DBMS.
    pub has_lesser_value: bool,
    /// True iff max_value is greater than the maximum value in the current page.
    /// This comparison should be performed by the DBMS.
    pub has_greater_value: bool,
}

impl KeysetPaginationResultForString {
    /// From this result object and the options object used for the latest request,
    /// construct a new options object to request the "next" page.
    pub fn create_next(&self, base_options: &KeysetPaginationOptionsForString) -> KeysetPaginationOptionsForString {
        let mut next_options = base_options.clone();
        next_options.sort_order = SortOrder::Ascending;
        next_options.start_value = match base_options.sort_order {
            SortOrder::Ascending => self.next_value.clone(),
            SortOrder::Descending => base_options.start_value.clone(),
        };
        next_options
    }

    /// From this result object and the options object used for the latest request,
    /// construct a new options object to request the "previous" page.
    pub fn create_previous(&self, base_options: &KeysetPaginationOptionsForString) -> KeysetPaginationOptionsForString {
        let mut prev_options = base_options.clone();
        prev_options.sort_order = SortOrder::Descending;
        prev_options.start_value = match base_options.sort_order {
            SortOrder::Ascending => base_options.start_value.clone(),
            SortOrder::Descending => self.next_value.clone(),
        };
        prev_options
    }
}
