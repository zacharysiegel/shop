use chrono::{DateTime, Utc};
use int_enum::IntEnum;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::{ShopEntity, ShopModel, ShopSerial};
use crate::error::ShopError;
use crate::server::JsonHttpResponse;

#[derive(Debug)]
pub struct CustomerEntity {
    pub id: Uuid,
    pub email_address: String,
    pub phone_number: Option<String>,
    pub password_hash: String,
    pub display_name: String,
    pub role: i32,
    pub status: i32,
    pub shipping_street_address: Option<String>,
    pub shipping_municipality: Option<String>,
    pub shipping_district: Option<String> ,
    pub shipping_postal_area: Option<String>,
    pub shipping_country: Option<String>,
    pub billing_street_address: Option<String>,
    pub billing_municipality: Option<String>,
    pub billing_district: Option<String>,
    pub billing_postal_area: Option<String>,
    pub billing_country: Option<String>,
    pub created: DateTime<Utc>,
    pub updated: DateTime<Utc>,
}

impl ShopEntity for CustomerEntity {
    type Model = CustomerModel;
}

/// On storing addresses: https://web.archive.org/web/20191008203135/http://www.endswithsaurus.com/2009/07/lesson-in-address-storage.html
#[derive(Debug)]
pub struct CustomerModel {
    pub id: Uuid,
    pub email_address: String,
    pub phone_number: Option<String>,
    pub password_hash: String,
    pub display_name: String,
    pub role: CustomerRole,
    pub status: CustomerStatus,
    pub shipping_street_address: Option<String>,
    pub shipping_municipality: Option<String>, // minor + major
    pub shipping_district: Option<String> ,
    pub shipping_postal_area: Option<String>,
    pub shipping_country: Option<String>,
    pub billing_street_address: Option<String>, // street number + number suffix + street name + street type + direction + address type + sub id
    pub billing_municipality: Option<String>, // minor + major
    pub billing_district: Option<String>,
    pub billing_postal_area: Option<String>,
    pub billing_country: Option<String>,
    pub created: DateTime<Utc>,
    pub updated: DateTime<Utc>,
}

impl ShopModel for CustomerModel {
    type Entity = CustomerEntity;
    type Serial = CustomerSerial;

    fn to_serial(&self) -> Self::Serial {
        todo!()
    }

    fn try_from_serial(serial: &Self::Serial) -> Result<Self, ShopError> {
        todo!()
    }

    fn to_entity(&self) -> Self::Entity {
        todo!()
    }

    fn try_from_entity(entity: &Self::Entity) -> Result<Self, ShopError> {
        todo!()
    }
}

#[derive(IntEnum, Debug, Clone)]
#[repr(u8)]
pub enum CustomerRole {
    Guest = 0,
    User,
    Administrator,
    Developer,
}

#[derive(IntEnum, Debug, Clone)]
#[repr(u8)]
pub enum CustomerStatus {
    Disabled = 0,
    Enabled,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CustomerSerial {
    #[serde(skip_deserializing, default = "crate::random_uuid")]
    pub id: Uuid,
    pub email_address: String,
    pub phone_number: Option<String>,
    pub password_hash: String,
    pub display_name: String,
    pub role: u8,
    pub status: u8,
    pub shipping_street_address: Option<String>,
    pub shipping_municipality: Option<String>,
    pub shipping_district: Option<String> ,
    pub shipping_postal_area: Option<String>,
    pub shipping_country: Option<String>,
    pub billing_street_address: Option<String>,
    pub billing_municipality: Option<String>,
    pub billing_district: Option<String>,
    pub billing_postal_area: Option<String>,
    pub billing_country: Option<String>,
    pub created: DateTime<Utc>,
    pub updated: DateTime<Utc>,
}

impl ShopSerial for CustomerSerial {
    type Model = CustomerModel;
}

impl JsonHttpResponse for CustomerSerial {}
impl JsonHttpResponse for Vec<CustomerSerial> {}
