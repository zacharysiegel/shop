use crate::error::ShopError;
use crate::{ShopEntity, ShopModel, ShopSerial};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct CategoryEntity {
    pub id: Uuid,
    pub ebay_category_id: String,
    pub ebay_category_tree_id: String,
    pub ebay_category_tree_version: String,
    pub ebay_category_name: String,
}

impl ShopEntity for CategoryEntity {
    type Model = Category;
}

pub type Category = CategoryEntity;
impl ShopModel for Category {
    type Entity = CategoryEntity;
    type Serial = CategorySerial;

    fn to_serial(&self) -> Self::Serial {
        CategorySerial {
            id: self.id.clone(),
            ebay_category_id: self.ebay_category_id.clone(),
            ebay_category_tree_id: self.ebay_category_tree_id.clone(),
            ebay_category_tree_version: self.ebay_category_tree_version.clone(),
            ebay_category_name: self.ebay_category_name.clone(),
        }
    }

    fn try_from_serial(serial: &Self::Serial) -> Result<Self, ShopError> {
        Ok(Category {
            id: serial.id.clone(),
            ebay_category_id: serial.ebay_category_id.clone(),
            ebay_category_tree_id: serial.ebay_category_tree_id.clone(),
            ebay_category_tree_version: serial.ebay_category_tree_version.clone(),
            ebay_category_name: serial.ebay_category_name.clone(),
        })
    }

    fn to_entity(&self) -> Self::Entity {
        self.clone()
    }

    fn try_from_entity(entity: &Self::Entity) -> Result<Self, ShopError> {
        Ok(entity.clone())
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CategorySerial {
    #[serde(default)]
    pub id: Uuid,
    pub ebay_category_id: String,
    pub ebay_category_tree_id: String,
    pub ebay_category_tree_version: String,
    pub ebay_category_name: String,
}

impl ShopSerial for CategorySerial {
    type Model = Category;
}
