use std::cmp::PartialEq;
use crate::ShopEntity;
use actix_web::guard;
use serde::{Deserialize, Serialize};
use crate::error::ShopError;

pub fn pagination_guard(ctx: &guard::GuardContext) -> bool {
    ctx.head()
        .uri
        .query()
        .unwrap_or("")
        .contains("page_size")
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Direction {
    #[serde(rename = "asc")]
    Ascending,
    #[serde(rename = "desc")]
    Descending,
}

impl Default for Direction {
    fn default() -> Self {
        Direction::Ascending
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeysetPaginationOptionsForString {
    /// Maximum number of elements in a returned page.
    pub max_page_size: u32,
    /// Ascend or descend in the relation from your current position.
    pub direction: Direction,
    /// If ascending, this is the preceding element to the desired page.
    /// If descending, this is the maximal element in the desired page.
    /// If none, returns the first page.
    pub start_value: Option<String>,
}

impl KeysetPaginationOptionsForString {
    pub fn validated(self) -> Result<Self, ShopError> {
        if self.max_page_size.overflowing_add(1).1 {
            return Err(ShopError { message: format!("Maximum page size exceeds maximum value; [{}]", u32::MAX - 1) });
        } else if self.max_page_size == 0 {
            return Err(ShopError { message: "Maximum page size cannot be zero;".to_string() });
        }
        
        if self.start_value.is_none() && self.direction == Direction::Descending {
            return Err(ShopError { message: "Unspecified start value cannot request a descending page;".to_string() });
        }

        Ok(self)
    }
}

impl Default for KeysetPaginationOptionsForString {
    fn default() -> Self {
        KeysetPaginationOptionsForString {
            max_page_size: 50,
            direction: Direction::Ascending,
            start_value: None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KeysetPaginationResultForString {
    pub page_size: u32,
    pub l_value: Option<String>,
    pub r_value: Option<String>,
    /// The maximum value for the entire table.
    pub relation_max_value: Option<String>,
    /// The minimum value for the entire table.
    pub relation_min_value: Option<String>,
    /// True iff min_value is less than the minimum value in the current page.
    /// This comparison should be performed by the DBMS.
    pub has_lesser_value: bool,
    /// True iff max_value is greater than the maximum value in the current page.
    /// This comparison should be performed by the DBMS.
    pub has_greater_value: bool,
}

impl KeysetPaginationResultForString {
    pub fn create(&self, direction: &Direction, max_page_size: &u32) -> KeysetPaginationOptionsForString {
        KeysetPaginationOptionsForString {
            max_page_size: max_page_size.clone(),
            direction: direction.clone(),
            start_value: match direction {
                Direction::Ascending => self.r_value.clone(),
                Direction::Descending => self.l_value.clone(),
            },
        }
    }

    /// entities: This vector should be retrieved from the DBMS in ascending sorted order.
    pub fn from_entities<EntityT>(
        entities: Vec<EntityT>,
        relation_min_entity: Option<EntityT>,
        relation_max_entity: Option<EntityT>,
        getter: fn(EntityT) -> String,
        max_page_size: usize,
    ) -> (Vec<EntityT>, KeysetPaginationResultForString)
    where
        EntityT: ShopEntity + Clone,
    {
        let relation_max_value: Option<String> = relation_max_entity.map(getter);
        let relation_min_value: Option<String> = relation_min_entity.map(getter);
        debug_assert!(
            entities.len() == 0 && relation_max_value.is_none() && relation_min_value.is_none()
                || entities.len() > 0 && relation_max_value.is_some() && relation_min_value.is_some()
        );

        let l_value: Option<String> = {
            if entities.len() == 0 {
                None
            } else {
                let first_value = entities.get(0)
                    .map(|val| val.clone())
                    .map(getter);
                debug_assert!(first_value.is_some()); // Vector is not empty.

                if entities.len() == max_page_size + 1 {
                    first_value
                } else {
                    // Vector has been truncated. It is at the edge of the relation.
                    if first_value == relation_min_value {
                        None
                    } else {
                        first_value
                    }
                }
            }
        };
        let r_value: Option<String> = {
            if entities.len() == 0 {
                None
            } else {
                let last_value = entities.get(entities.len() - 1)
                    .map(|val| val.clone())
                    .map(getter);
                debug_assert!(last_value.is_some()); // Vector is not empty.

                if last_value == relation_max_value {
                    None
                } else {
                    last_value
                }
            }
        };

        let has_greater_value: bool = r_value.is_some();
        let has_lesser_value: bool = l_value.is_some();

        let page: Vec<EntityT> = if entities.len() == max_page_size + 1 {
            entities[1..].to_vec()
        } else {
            // Page is truncated
            if l_value.is_some() {
                entities[1..].to_vec()
            } else {
                entities[..].to_vec()
            }
        };
        let page_size: u32 = page.len() as u32;

        (
            page,
            KeysetPaginationResultForString {
                page_size,
                l_value,
                r_value,
                relation_max_value,
                relation_min_value,
                has_greater_value,
                has_lesser_value,
            }
        )
    }
}
