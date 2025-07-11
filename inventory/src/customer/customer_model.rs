use crate::error::ShopError;
use crate::object::JsonHttpResponse;
use crate::{object, try_from_repr, ShopEntity, ShopModel, ShopSerial};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use strum::FromRepr;
use uuid::Uuid;

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
    pub shipping_district: Option<String>,
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
    type Model = Customer;
}

/// On storing addresses: https://web.archive.org/web/20191008203135/http://www.endswithsaurus.com/2009/07/lesson-in-address-storage.html
#[derive(Debug)]
pub struct Customer {
    pub id: Uuid,
    pub email_address: String,
    pub phone_number: Option<String>,
    pub password_hash: String,
    pub display_name: String,
    pub role: CustomerRole,
    pub status: CustomerStatus,
    pub shipping_street_address: Option<String>,
    pub shipping_municipality: Option<String>, // minor + major
    pub shipping_district: Option<String>,
    pub shipping_postal_area: Option<String>,
    pub shipping_country: Option<String>,
    pub billing_street_address: Option<String>, // street number + number suffix + street name + street type + direction + address type + sub id
    pub billing_municipality: Option<String>,   // minor + major
    pub billing_district: Option<String>,
    pub billing_postal_area: Option<String>,
    pub billing_country: Option<String>,
    pub created: DateTime<Utc>,
    pub updated: DateTime<Utc>,
}

impl ShopModel for Customer {
    type Entity = CustomerEntity;
    type Serial = CustomerSerial;

    fn to_serial(&self) -> Self::Serial {
        Self::Serial {
            id: self.id.clone(),
            email_address: self.email_address.clone(),
            phone_number: self.phone_number.clone(),
            password_hash: self.password_hash.clone(),
            display_name: self.display_name.clone(),
            role: self.role.clone() as u8,
            status: self.status.clone() as u8,
            shipping_street_address: self.shipping_street_address.clone(),
            shipping_municipality: self.shipping_municipality.clone(),
            shipping_district: self.shipping_district.clone(),
            shipping_postal_area: self.shipping_postal_area.clone(),
            shipping_country: self.shipping_country.clone(),
            billing_street_address: self.billing_street_address.clone(),
            billing_municipality: self.billing_municipality.clone(),
            billing_district: self.billing_district.clone(),
            billing_postal_area: self.billing_postal_area.clone(),
            billing_country: self.billing_country.clone(),
            created: self.created.clone(),
            updated: self.updated.clone(),
        }
    }

    fn try_from_serial(serial: &Self::Serial) -> Result<Self, ShopError> {
        Ok(Self {
            id: object::random_uuid(),
            email_address: serial.email_address.clone(),
            phone_number: serial.phone_number.clone(),
            password_hash: serial.password_hash.clone(),
            display_name: serial.display_name.clone(),
            role: CustomerRole::try_from_repr(serial.role.clone())?,
            status: CustomerStatus::try_from_repr(serial.status.clone())?,
            shipping_street_address: serial.shipping_street_address.clone(),
            shipping_municipality: serial.shipping_municipality.clone(),
            shipping_district: serial.shipping_district.clone(),
            shipping_postal_area: serial.shipping_postal_area.clone(),
            shipping_country: serial.shipping_country.clone(),
            billing_street_address: serial.billing_street_address.clone(),
            billing_municipality: serial.billing_municipality.clone(),
            billing_district: serial.billing_district.clone(),
            billing_postal_area: serial.billing_postal_area.clone(),
            billing_country: serial.billing_country.clone(),
            created: serial.created.clone(),
            updated: serial.updated.clone(),
        })
    }

    fn to_entity(&self) -> Self::Entity {
        Self::Entity {
            id: self.id.clone(),
            email_address: self.email_address.clone(),
            phone_number: self.phone_number.clone(),
            password_hash: self.password_hash.clone(),
            display_name: self.display_name.clone(),
            role: self.role.clone() as i32,
            status: self.status.clone() as i32,
            shipping_street_address: self.shipping_street_address.clone(),
            shipping_municipality: self.shipping_municipality.clone(),
            shipping_district: self.shipping_district.clone(),
            shipping_postal_area: self.shipping_postal_area.clone(),
            shipping_country: self.shipping_country.clone(),
            billing_street_address: self.billing_street_address.clone(),
            billing_municipality: self.billing_municipality.clone(),
            billing_district: self.billing_district.clone(),
            billing_postal_area: self.billing_postal_area.clone(),
            billing_country: self.billing_country.clone(),
            created: self.created.clone(),
            updated: self.updated.clone(),
        }
    }

    fn try_from_entity(entity: &Self::Entity) -> Result<Self, ShopError> {
        Ok(Self {
            id: entity.id.clone(),
            email_address: entity.email_address.clone(),
            phone_number: entity.phone_number.clone(),
            password_hash: entity.password_hash.clone(),
            display_name: entity.display_name.clone(),
            role: CustomerRole::try_from_repr(entity.role.clone() as u8)?,
            status: CustomerStatus::try_from_repr(entity.status.clone() as u8)?,
            shipping_street_address: entity.shipping_street_address.clone(),
            shipping_municipality: entity.shipping_municipality.clone(),
            shipping_district: entity.shipping_district.clone(),
            shipping_postal_area: entity.shipping_postal_area.clone(),
            shipping_country: entity.shipping_country.clone(),
            billing_street_address: entity.billing_street_address.clone(),
            billing_municipality: entity.billing_municipality.clone(),
            billing_district: entity.billing_district.clone(),
            billing_postal_area: entity.billing_postal_area.clone(),
            billing_country: entity.billing_country.clone(),
            created: entity.created.clone(),
            updated: entity.updated.clone(),
        })
    }
}

#[derive(Debug, Clone, FromRepr)]
#[repr(u8)]
pub enum CustomerRole {
    Guest = 0,
    User,
    Administrator,
    Developer,
}

try_from_repr!(CustomerRole<u8>);

#[derive(Debug, Clone, FromRepr)]
#[repr(u8)]
pub enum CustomerStatus {
    Disabled = 0,
    Enabled,
}

try_from_repr!(CustomerStatus<u8>);

#[derive(Debug, Serialize, Deserialize)]
pub struct CustomerSerial {
    #[serde(default)]
    pub id: Uuid,
    pub email_address: String,
    pub phone_number: Option<String>,
    pub password_hash: String,
    pub display_name: String,
    pub role: u8,
    pub status: u8,
    pub shipping_street_address: Option<String>,
    pub shipping_municipality: Option<String>,
    pub shipping_district: Option<String>,
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
    type Model = Customer;
}

impl JsonHttpResponse for CustomerSerial {}
impl JsonHttpResponse for Vec<CustomerSerial> {}
