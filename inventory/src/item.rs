use crate::server::JsonHttpResponse;
use crate::InventoryResource;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug)]
pub struct Item { // todo: rename all of these to ...Entity
	id: Uuid,
	product_id: Uuid,
	inventory_location_id: Uuid,
	condition: i32,
	status: i32,
	price_cents: i64,
	priority: i32,
	note: Option<String>,
	acquisition_datetime: DateTime<Utc>,
	acquisition_price_cents: Option<i64>,
	acquisition_location: Option<String>,
	created: DateTime<Utc>,
	updated: DateTime<Utc>,
}

impl InventoryResource for Item {
	type Serializable = ItemSerial;

	fn to_serial(&self) -> Self::Serializable {
		ItemSerial {
            id: self.id.clone(),
            product_id: self.product_id.clone(),
            inventory_location_id: self.inventory_location_id.clone(),
            condition: self.condition,
            status: self.status.clone(),
            price_cents: self.price_cents.clone(),
            priority: self.priority.clone(),
            note: self.note.clone(),
            acquisition_datetime: self.acquisition_datetime.clone(),
            acquisition_price_cents: self.acquisition_price_cents.clone(),
            acquisition_location: self.acquisition_location.clone(),
            created: self.created.clone(),
            updated: self.updated.clone(),
        }
	}

	fn from_serial(serializable: &Self::Serializable) -> Self {
		Item {
            id: serializable.id.clone(),
            product_id: serializable.product_id.clone(),
            inventory_location_id: serializable.inventory_location_id.clone(),
            condition: serializable.condition,
            status: serializable.status.clone(),
            price_cents: serializable.price_cents.clone(),
            priority: serializable.priority.clone(),
            note: serializable.note.clone(),
            acquisition_datetime: serializable.acquisition_datetime.clone(),
            acquisition_price_cents: serializable.acquisition_price_cents.clone(),
            acquisition_location: serializable.acquisition_location.clone(),
            created: serializable.created.clone(),
            updated: serializable.updated.clone(),
        }
	}
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ItemSerial {
    #[serde(skip_deserializing, default = "crate::random_uuid")]
    id: Uuid,
    product_id: Uuid,
    inventory_location_id: Uuid,
    condition: i32,
    status: i32,
    price_cents: i64,
    priority: i32,
    note: Option<String>,
    acquisition_datetime: DateTime<Utc>,
    acquisition_price_cents: Option<i64>,
    acquisition_location: Option<String>,
    created: DateTime<Utc>,
    updated: DateTime<Utc>,
}

impl JsonHttpResponse for ItemSerial {}
impl JsonHttpResponse for Vec<ItemSerial> {}

mod db {}

pub mod route {}
