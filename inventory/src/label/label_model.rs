use crate::error::ShopError;
use crate::object::JsonHttpResponse;
use crate::{object, ShopEntity, ShopModel, ShopSerial};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct LabelEntity {
    pub id: Uuid,
    pub display_name: String,
    pub internal_name: String,
}

impl ShopEntity for LabelEntity {
    type Model = Label;
}

pub type Label = LabelEntity;

impl ShopModel for Label {
    type Entity = LabelEntity;
    type Serial = LabelSerial;

    fn to_serial(&self) -> Self::Serial {
        LabelSerial {
            id: self.id.clone(),
            display_name: self.display_name.clone(),
            internal_name: self.internal_name.clone(),
        }
    }

    fn try_from_serial(serial: &Self::Serial) -> Result<Self, ShopError> {
        Ok(LabelEntity {
            id: object::random_uuid(),
            display_name: serial.display_name.clone(),
            internal_name: serial.internal_name.clone(),
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
pub struct LabelSerial {
    #[serde(default)]
    pub id: Uuid,
    pub display_name: String,
    pub internal_name: String,
}

impl ShopSerial for LabelSerial {
    type Model = Label;
}
impl JsonHttpResponse for LabelSerial {}
impl JsonHttpResponse for Vec<LabelSerial> {}
