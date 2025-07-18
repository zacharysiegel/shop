use crate::error::ShopError;
use crate::object::JsonHttpResponse;
use crate::{object, ShopEntity, ShopModel, ShopSerial};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct CategoryEntity {
    pub id: Uuid,
    pub display_name: String,
    pub internal_name: String,
    pub parent_id: Option<Uuid>,
    pub ebay_category_id: Uuid,
}

impl ShopEntity for CategoryEntity {
    type Model = Category;
}

pub type Category = CategoryEntity;

impl ShopModel for Category {
    type Entity = Self;
    type Serial = CategorySerial;

    fn to_serial(&self) -> CategorySerial {
        CategorySerial {
            id: self.id.clone(),
            display_name: self.display_name.clone(),
            internal_name: self.internal_name.clone(),
            parent_id: self.parent_id.clone(),
            ebay_category_id: self.ebay_category_id.clone(),
        }
    }

    fn try_from_serial(serial: &CategorySerial) -> Result<CategoryEntity, ShopError> {
        Ok(CategoryEntity {
            id: object::random_uuid(),
            display_name: serial.display_name.clone(),
            internal_name: serial.internal_name.clone(),
            parent_id: serial.parent_id.clone(),
            ebay_category_id: serial.ebay_category_id.clone(),
        })
    }

    fn to_entity(&self) -> Self::Entity {
        self.clone()
    }

    fn try_from_entity(entity: &Self::Entity) -> Result<CategoryEntity, ShopError> {
        Ok(entity.clone())
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CategorySerial {
    #[serde(default)]
    pub id: Uuid,
    pub display_name: String,
    pub internal_name: String,
    pub parent_id: Option<Uuid>,
    pub ebay_category_id: Uuid,
}

impl ShopSerial for CategorySerial {
    type Model = Category;
}
impl JsonHttpResponse for CategorySerial {}
impl JsonHttpResponse for Vec<CategorySerial> {}
