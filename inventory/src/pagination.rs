use crate::error::ShopError;
use crate::ShopEntity;
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
        next_options.start_value = self.next_value.clone();
        next_options
    }

    /// From this result object and the options object used for the latest request,
    /// construct a new options object to request the "previous" page.
    pub fn create_previous(&self, base_options: &KeysetPaginationOptionsForString) -> KeysetPaginationOptionsForString {
        let mut prev_options = base_options.clone();
        prev_options.sort_order = SortOrder::Descending;
        prev_options.start_value = self.start_value.clone();
        prev_options
    }

    pub fn from_entities<EntityT>(
        all_entities: Vec<EntityT>,
        min_entity: Option<EntityT>,
        max_entity: Option<EntityT>,
        getter: fn(EntityT) -> String,
        max_page_size: usize,
        sort_order: SortOrder,
    ) -> (Vec<EntityT>, KeysetPaginationResultForString)
    where
        EntityT: ShopEntity + Clone,
    {
        let max_value: Option<String> = max_entity.map(getter);
        let min_value: Option<String> = min_entity.map(getter);
        let start_value: Option<String> = all_entities
            .get(0)
            .map(|val| val.clone())
            .map(getter);
        let next_value = if all_entities.len() <= max_page_size {
            None
        } else {
            all_entities
                .get(match sort_order {
                    SortOrder::Ascending => all_entities.len() - 1,
                    SortOrder::Descending => 0,
                })
                .map(|val| val.clone())
                .map(getter)
        };

        debug_assert!(all_entities.len() == 0 && max_value.is_none() && min_value.is_none()
            || all_entities.len() > 0 && max_value.is_some() && min_value.is_some());

        // Note: If the page is empty, all *_value objects will be none, so will all equal each other, producing false
        let has_greater_value: bool = match sort_order {
            SortOrder::Ascending => next_value.is_some(),
            SortOrder::Descending => start_value != max_value,
        };
        let has_lesser_value: bool = match sort_order {
            SortOrder::Ascending => start_value != min_value,
            SortOrder::Descending => next_value.is_some(),
        };

        let page: Vec<EntityT> = if all_entities.len() <= max_page_size {
            all_entities
        } else {
            debug_assert!(all_entities.len() == max_page_size.wrapping_add(1));
            match sort_order {
                SortOrder::Ascending => all_entities[0..max_page_size].to_vec(),
                SortOrder::Descending => all_entities[1..max_page_size + 1].to_vec(),
            }
        };
        let page_size = page.len() as u32;
        (
            page,
            KeysetPaginationResultForString {
                page_size,
                start_value,
                next_value,
                max_value,
                min_value,
                has_greater_value,
                has_lesser_value,
            }
        )
    }
}
