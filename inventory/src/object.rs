use crate::error::ShopError;
use actix_web::http::StatusCode;
use actix_web::{HttpResponse, HttpResponseBuilder};
use rand::prelude::ThreadRng;
use rand::RngCore;
use serde::Serialize;
use std::fmt::Debug;
use uuid::Uuid;

pub fn random_uuid() -> Uuid {
    let mut rng: ThreadRng = rand::rng();
    let mut random_bytes: [u8; 128 >> 3] = [0; 128 >> 3]; // 128 bits
    rng.fill_bytes(&mut random_bytes); // ThreadRng::fill_bytes never panics

    assert_eq!(random_bytes.len(), 16);
    Uuid::from_slice(&random_bytes[..]).unwrap() // Err is only returned for non 16 byte length
}

/// Standard mappings between structs at the db-level, service-level, and api-level
pub trait ShopModel: Sized + Debug {
    type Entity: ShopEntity<Model=Self>;
    type Serial: ShopSerial<Model=Self>;

    fn to_serial(&self) -> Self::Serial;
    fn try_from_serial(serial: &Self::Serial) -> Result<Self, ShopError>;

    fn to_entity(&self) -> Self::Entity;
    fn try_from_entity(entity: &Self::Entity) -> Result<Self, ShopError>;
}

pub trait ShopEntity: Sized + Debug {
    type Model: ShopModel<Entity=Self>;

    fn try_to_model(&self) -> Result<Self::Model, ShopError> {
        Self::Model::try_from_entity(self)
    }
    fn from_model(model: &Self::Model) -> Self {
        Self::Model::to_entity(model)
    }
}

pub trait ShopSerial: Sized + Debug {
    type Model: ShopModel<Serial=Self>;

    fn try_to_model(&self) -> Result<Self::Model, ShopError> {
        Self::Model::try_from_serial(self)
    }
    fn from_model(model: &Self::Model) -> Self {
        Self::Model::to_serial(model)
    }
}

pub trait JsonHttpResponse
where
    Self: Sized + Serialize,
{
    fn to_http_response(&self) -> HttpResponse {
        let Ok(json) = serde_json::to_string(&self) else {
            return HttpResponseBuilder::new(StatusCode::INTERNAL_SERVER_ERROR).body(());
        };
        HttpResponseBuilder::new(StatusCode::OK).body(json)
    }
}
