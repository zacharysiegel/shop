use crate::error::ShopError;
use crate::object::JsonHttpResponse;
use crate::{object, try_from_repr, ShopEntity, ShopModel, ShopSerial};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use strum::FromRepr;
use uuid::Uuid;

#[derive(Debug)]
pub struct PurchaseEntity {
    pub id: Uuid,
    pub marketplace_id: Uuid,
    pub external_id: Option<String>,
    pub customer_id: Option<Uuid>,
    pub contact_email_address: String,
    pub listing_id: Uuid,
    pub status: i32,
    pub cost_subtotal_cents: i64,
    pub cost_tax_cents: i64,
    pub cost_shipping_cents: i64,
    pub cost_discount_cents: i64,
    pub seller_cost_total_cents: i64,
    pub shipping_method: i32,
    pub payment_method: i32,
    pub note: Option<String>,
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

impl ShopEntity for PurchaseEntity {
    type Model = PurchaseModel;
}

#[derive(Debug)]
pub struct PurchaseModel {
    pub id: Uuid,
    pub marketplace_id: Uuid,
    pub external_id: Option<String>,
    pub customer_id: Option<Uuid>,
    pub contact_email_address: String,
    pub listing_id: Uuid,
    pub status: PurchaseStatus,
    pub cost_subtotal_cents: i64,
    pub cost_tax_cents: i64,
    pub cost_shipping_cents: i64,
    pub cost_discount_cents: i64,
    pub seller_cost_total_cents: i64, // Represents the total fees we paid to fulfill this transaction (e.g. marketplace, payment processor)
    pub shipping_method: ShippingMethod,
    pub payment_method: PaymentMethod,
    pub note: Option<String>,
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

impl ShopModel for PurchaseModel {
    type Entity = PurchaseEntity;
    type Serial = PurchaseSerial;

    fn to_serial(&self) -> Self::Serial {
        PurchaseSerial {
            id: self.id.clone(),
            marketplace_id: self.marketplace_id.clone(),
            external_id: self.external_id.clone(),
            customer_id: self.customer_id.clone(),
            contact_email_address: self.contact_email_address.clone(),
            listing_id: self.listing_id.clone(),
            status: self.status.clone() as u8,
            cost_subtotal_cents: self.cost_subtotal_cents.clone(),
            cost_tax_cents: self.cost_tax_cents.clone(),
            cost_shipping_cents: self.cost_shipping_cents.clone(),
            cost_discount_cents: self.cost_discount_cents.clone(),
            seller_cost_total_cents: self.seller_cost_total_cents.clone(),
            shipping_method: self.shipping_method.clone() as u8,
            payment_method: self.payment_method.clone() as u8,
            note: self.note.clone(),
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
        Ok(PurchaseModel {
            id: object::random_uuid(),
            marketplace_id: serial.marketplace_id.clone(),
            external_id: serial.external_id.clone(),
            customer_id: serial.customer_id.clone(),
            contact_email_address: serial.contact_email_address.clone(),
            listing_id: serial.listing_id.clone(),
            status: PurchaseStatus::try_from_repr(serial.status.clone())?,
            cost_subtotal_cents: serial.cost_subtotal_cents.clone(),
            cost_tax_cents: serial.cost_tax_cents.clone(),
            cost_shipping_cents: serial.cost_shipping_cents.clone(),
            cost_discount_cents: serial.cost_discount_cents.clone(),
            seller_cost_total_cents: serial.seller_cost_total_cents.clone(),
            shipping_method: ShippingMethod::try_from_repr(
                serial.shipping_method.clone(),
            )?,
            payment_method: PaymentMethod::try_from_repr(
                serial.payment_method.clone(),
            )?,
            note: serial.note.clone(),
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
        PurchaseEntity {
            id: self.id.clone(),
            marketplace_id: self.marketplace_id.clone(),
            external_id: self.external_id.clone(),
            customer_id: self.customer_id.clone(),
            contact_email_address: self.contact_email_address.clone(),
            listing_id: self.listing_id.clone(),
            status: i32::from(self.status.clone() as u8),
            cost_subtotal_cents: self.cost_subtotal_cents.clone(),
            cost_tax_cents: self.cost_tax_cents.clone(),
            cost_shipping_cents: self.cost_shipping_cents.clone(),
            cost_discount_cents: self.cost_discount_cents.clone(),
            seller_cost_total_cents: self.seller_cost_total_cents.clone(),
            shipping_method: i32::from(self.shipping_method.clone() as u8),
            payment_method: i32::from(self.payment_method.clone() as u8),
            note: self.note.clone(),
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
        Ok(PurchaseModel {
            id: entity.id.clone(),
            marketplace_id: entity.marketplace_id.clone(),
            external_id: entity.external_id.clone(),
            customer_id: entity.customer_id.clone(),
            contact_email_address: entity.contact_email_address.clone(),
            listing_id: entity.listing_id.clone(),
            status: PurchaseStatus::try_from_repr(entity.status.clone() as u8)?,
            cost_subtotal_cents: entity.cost_subtotal_cents.clone(),
            cost_tax_cents: entity.cost_tax_cents.clone(),
            cost_shipping_cents: entity.cost_shipping_cents.clone(),
            cost_discount_cents: entity.cost_discount_cents.clone(),
            seller_cost_total_cents: entity.seller_cost_total_cents.clone(),
            shipping_method: ShippingMethod::try_from_repr(
                entity.shipping_method.clone() as u8,
            )?,
            payment_method: PaymentMethod::try_from_repr(
                entity.payment_method.clone() as u8,
            )?,
            note: entity.note.clone(),
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
pub enum PurchaseStatus {
    Fulfilled = 0,
    Cancelled,
}

try_from_repr!(PurchaseStatus<u8>);

#[derive(Debug, Clone, FromRepr)]
#[repr(u8)]
pub enum ShippingMethod {
    Pickup = 0,
    Shipping,
}

try_from_repr!(ShippingMethod<u8>);

#[derive(Debug, Clone, FromRepr)]
#[repr(u8)]
pub enum PaymentMethod {
    Cash = 0,
    Check,
    Credit,
    Debit,
}

try_from_repr!(PaymentMethod<u8>);

#[derive(Debug, Serialize, Deserialize)]
pub struct PurchaseSerial {
    #[serde(default)]
    pub id: Uuid,
    pub marketplace_id: Uuid,
    pub external_id: Option<String>,
    pub customer_id: Option<Uuid>,
    pub contact_email_address: String,
    pub listing_id: Uuid,
    pub status: u8,
    pub cost_subtotal_cents: i64,
    pub cost_tax_cents: i64,
    pub cost_shipping_cents: i64,
    pub cost_discount_cents: i64,
    pub seller_cost_total_cents: i64, // Represents the total fees we paid to fulfill this transaction (e.g. marketplace, payment processor)
    pub shipping_method: u8,
    pub payment_method: u8,
    pub note: Option<String>,
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

impl ShopSerial for PurchaseSerial {
    type Model = PurchaseModel;
}

impl JsonHttpResponse for PurchaseSerial {}
impl JsonHttpResponse for Vec<PurchaseSerial> {}
