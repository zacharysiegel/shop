use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::{ShopEntity, ShopModel, ShopSerial};
use crate::error::ShopError;
use crate::server::JsonHttpResponse;

#[derive(Debug, Clone)]
pub struct MetricCounter {
    pub id: Uuid,
    pub internal_name: String,
    pub object_id: Option<String>,
    pub value: i64,
}

impl ShopEntity for MetricCounter {
    type Model = MetricCounter;
}
impl ShopModel for MetricCounter {
    type Entity = MetricCounter;
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
        Ok(MetricCounter {
            id: serial.id.clone(),
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
    #[serde(skip_deserializing, default = "crate::random_uuid")]
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
