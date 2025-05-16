use crate::error::ShopError;
use crate::object::JsonHttpResponse;
use crate::{object, ShopEntity, ShopModel, ShopSerial};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct MetricCounterEntity {
    pub id: Uuid,
    pub internal_name: String,
    pub object_id: Option<String>,
    pub value: i64,
}

impl ShopEntity for MetricCounterEntity {
    type Model = MetricCounter;
}

pub type MetricCounter = MetricCounterEntity;

impl ShopModel for MetricCounter {
    type Entity = MetricCounterEntity;
    type Serial = MetricCounterSerial;

    fn to_serial(&self) -> Self::Serial {
        MetricCounterSerial {
            id: self.id.clone(),
            internal_name: self.internal_name.clone(),
            object_id: self.object_id.clone(),
            value: self.value.clone(),
        }
    }

    fn try_from_serial(serial: &Self::Serial) -> Result<Self, ShopError> {
        Ok(MetricCounterEntity {
            id: object::random_uuid(),
            internal_name: serial.internal_name.clone(),
            object_id: serial.object_id.clone(),
            value: serial.value.clone(),
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
pub struct MetricCounterSerial {
    #[serde(default)]
    pub id: Uuid,
    pub internal_name: String,
    pub object_id: Option<String>,
    pub value: i64,
}

impl ShopSerial for MetricCounterSerial {
    type Model = MetricCounter;
}

impl JsonHttpResponse for MetricCounterSerial {}
impl JsonHttpResponse for Vec<MetricCounterSerial> {}
