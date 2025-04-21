use crate::error::ShopError;
use crate::server::JsonHttpResponse;
use crate::{ShopEntity, ShopModel, ShopSerial};
use chrono::{DateTime, Utc};
use int_enum::IntEnum;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub struct Item {
	pub id: Uuid,
	pub product_id: Uuid,
	pub inventory_location_id: Uuid,
	pub condition: ItemCondition,
	pub status: ItemStatus,
	pub price_cents: u32,
	pub priority: i32,
	pub note: Option<String>,
	pub acquisition_datetime: DateTime<Utc>,
	pub acquisition_price_cents: Option<u32>,
	pub acquisition_location: Option<String>,
	pub created: DateTime<Utc>,
	pub updated: DateTime<Utc>,
	// Add grading information?
	// Add refurbishment information?
}

/// Variants inspired by Ebay: https://www.ebay.com/help/selling/listings/creating-managing-listings/item-conditions-category.
/// See the "Movies & TV, Music, Video Games" section.
#[derive(IntEnum, Clone)] // todo: Reimplement this myself and remove the int-enum dependency
pub enum ItemCondition {
	Inapplicable = 0,
	BrandNew,
	LikeNew,
	VeryGood,
	Good,
	Acceptable,
	Digital,
}

impl ItemCondition {
	/// Wraps the IntEnum derived try_from implementation to return a result containing ShopError
	fn try_from_custom(v: isize) -> Result<Self, ShopError> {
		match ItemCondition::try_from(v) {
			Ok(variant) => Ok(variant),
			Err(value) => Err(ShopError {
				message: String::from(format!("Error parsing condition [{}]", value)),
			}),
		}
	}
}

#[derive(IntEnum, Clone)]
pub enum ItemStatus {
	/// Item is only partially constructed and expects modifications before publishing
	Incomplete = 0,
	/// Item is completely specified but is not yet listed on any marketplace
	CompleteUnlisted,
	/// Item is listed on any marketplace
	CompleteListed,
	/// A customer has requested the item be delisted from all marketplaces in anticipation of purchase.
	CustomerHoldListed,
	/// Item has been delisted from all marketplaces at the request of a potential buyer
	CustomerHoldDelisted,
	/// Item has been purchased but has not yet been delisted from all marketplaces
	PurchaseListed,
	/// Item has been delisted from all marketplaces after a purchase
	PurchasedDelisted,
	/// Item has been shipped
	Shipped,
	/// Item has been received by the customer (either via shipping or pickup)
	Received,
}

impl ItemStatus {
	/// Wraps the IntEnum derived try_from implementation to return a result containing ShopError
	fn try_from_custom(v: isize) -> Result<Self, ShopError> {
		match ItemStatus::try_from(v) {
			Ok(variant) => Ok(variant),
			Err(value) => Err(ShopError {
				message: String::from(format!("Error parsing status [{}]", value)),
			}),
		}
	}
}

impl ShopModel for Item {
	type Entity = ItemEntity;
	type Serial = ItemSerial;

	fn to_serial(&self) -> Self::Serial {
		ItemSerial {
			id: self.id.clone(),
			product_id: self.product_id.clone(),
			inventory_location_id: self.inventory_location_id.clone(),
			condition: self.condition.clone() as u8,
			status: self.status.clone() as u8,
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

	fn try_from_serial(serial: &Self::Serial) -> Result<Self, ShopError> {
		Ok(Item {
			id: serial.id.clone(),
			product_id: serial.product_id.clone(),
			inventory_location_id: serial.inventory_location_id.clone(),
			condition: ItemCondition::try_from_custom(serial.condition as isize)?,
			status: ItemStatus::try_from_custom(serial.status as isize)?,
			price_cents: serial.price_cents.clone(),
			priority: serial.priority.clone(),
			note: serial.note.clone(),
			acquisition_datetime: serial.acquisition_datetime.clone(),
			acquisition_price_cents: serial.acquisition_price_cents.clone(),
			acquisition_location: serial.acquisition_location.clone(),
			created: serial.created.clone(),
			updated: serial.updated.clone(),
		})
	}

	fn to_entity(&self) -> Self::Entity {
		ItemEntity {
			id: self.id.clone(),
			product_id: self.product_id.clone(),
			inventory_location_id: self.inventory_location_id.clone(),
			condition: self.condition.clone() as i32,
			status: self.status.clone() as i32,
			price_cents: i64::from(self.price_cents),
			priority: self.priority.clone(),
			note: self.note.clone(),
			acquisition_datetime: self.acquisition_datetime.clone(),
			acquisition_price_cents: self.acquisition_price_cents.map(|v| i64::from(v)),
			acquisition_location: self.acquisition_location.clone(),
			created: self.created.clone(),
			updated: self.updated.clone(),
		}
	}

	fn try_from_entity(entity: &Self::Entity) -> Result<Self, ShopError> {
		Ok(Item {
			id: entity.id.clone(),
			product_id: entity.product_id.clone(),
			inventory_location_id: entity.inventory_location_id.clone(),
			condition: ItemCondition::try_from_custom(entity.condition as isize)?,
			status: ItemStatus::try_from_custom(entity.status as isize)?,
			price_cents: 0,
			priority: 0,
			note: None,
			acquisition_datetime: Default::default(),
			acquisition_price_cents: None,
			acquisition_location: None,
			created: Default::default(),
			updated: Default::default(),
		})
	}
}

#[derive(Debug)]
pub struct ItemEntity {
	pub id: Uuid,
	pub product_id: Uuid,
	pub inventory_location_id: Uuid,
	pub condition: i32,
	pub status: i32,
	pub price_cents: i64,
	pub priority: i32,
	pub note: Option<String>,
	pub acquisition_datetime: DateTime<Utc>,
	pub acquisition_price_cents: Option<i64>,
	pub acquisition_location: Option<String>,
	pub created: DateTime<Utc>,
	pub updated: DateTime<Utc>,
}

impl ShopEntity for ItemEntity {
	type Model = Item;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ItemSerial {
	#[serde(skip_deserializing, default = "crate::random_uuid")]
	pub id: Uuid,
	pub product_id: Uuid,
	pub inventory_location_id: Uuid,
	pub condition: u8,
	pub status: u8,
	pub price_cents: u32,
	pub priority: i32,
	pub note: Option<String>,
	pub acquisition_datetime: DateTime<Utc>,
	pub acquisition_price_cents: Option<u32>,
	pub acquisition_location: Option<String>,
	pub created: DateTime<Utc>,
	pub updated: DateTime<Utc>,
}

impl ShopSerial for ItemSerial {
	type Model = Item;
}
impl JsonHttpResponse for ItemSerial {}
impl JsonHttpResponse for Vec<ItemSerial> {}
