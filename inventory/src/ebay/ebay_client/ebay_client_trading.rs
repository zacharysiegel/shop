use crate::ebay::ebay_client::ebay_client_shared::EBAY_BASE_URL;
use crate::ebay::ebay_client::{ebay_client_shared, ClientCredentialsResponse};
use crate::error::ShopError;
use crate::http;
use crate::http::HTTP_CLIENT;
use crate::item_image::ItemImage;
use reqwest::header::{CONTENT_LANGUAGE, CONTENT_TYPE};
use reqwest::Request;
use serde_json::json;

const TRADING_API_BASE_PATH: &str = "/ws/api.dll";
const ERROR_LANGUAGE: &str = "en_US";

enum WarningLevel {
    High,
    Low,
}

impl WarningLevel {
    pub fn get_serial_value(&self) -> String {
        match self {
            WarningLevel::High => "High".to_string(),
            WarningLevel::Low => "Low".to_string(),
        }
    }
}

enum PictureSetCodeType {
    Standard,
    Supersize,
}

impl PictureSetCodeType {
    pub fn get_serial_value(&self) -> String {
        match self {
            PictureSetCodeType::Standard => "Standard".to_string(),
            PictureSetCodeType::Supersize => "Supersize".to_string(),
        }
    }
}

pub async fn upload_image(item_image: ItemImage) -> Result<String, ShopError> {
    let application_token: ClientCredentialsResponse = super::get_application_token().await?;

    let mut body: String = format!(
        r#"
        <?xml version="1.0" encoding="utf-8"?>
        <UploadSiteHostedPicturesRequest xmlns="urn:ebay:apis:eBLBaseComponents">
            <RequesterCredentials>
                <ebl:eBayAuthToken xmlns:ebl="urn:ebay:apis:eBLBaseComponents">{}</ebl:eBayAuthToken>
            </RequesterCredentials>
            <!-- Call-specific Input Fields -->
            <!-- <ExternalPictureURL> anyURI </ExternalPictureURL> -->
            <PictureName>{}</PictureName>
            <PictureSet>{}</PictureSet>
            <PictureSystemVersion>2</PictureSystemVersion> <!-- "Only version 2 is valid" 07/01/2025 -->
            <PictureUploadPolicy>Add</PictureUploadPolicy>
            <!-- Standard Input Fields -->
            <ErrorLanguage>{}</ErrorLanguage>
            <MessageID>{0}</MessageID>
            <WarningLevel>{}</WarningLevel>
        </UploadSiteHostedPicturesRequest>
    "#,
        application_token.access_token,
        item_image.id,
        PictureSetCodeType::Standard.get_serial_value(),
        ERROR_LANGUAGE,
        WarningLevel::High.get_serial_value(),
    );
    // The upc array cannot take null values (from Option::None), so we need to dynamically insert
    if let Some(upc) = &product.upc {
        let upc_array = body.index_mut("product").index_mut("upc")
            .as_array_mut()
            .ok_or_else(|| ShopError::default())?;
        upc_array.push(json!(upc))
    }
    let body: String = serde_json::to_string(&body)
        .map_err(|e|
            ShopError::from_error("serializing inventory item", Box::new(e))
        )?;

    let request: Request = HTTP_CLIENT
        .put(format!("{}{}/inventory_item/{}", *EBAY_BASE_URL, crate::ebay::ebay_client::ebay_client_inventory::INVENTORY_API_BASE_PATH, item.id))
        .header(CONTENT_LANGUAGE, ebay_client_shared::EBAY_CONTENT_LANGUAGE)
        .header(CONTENT_TYPE, "application/json")
        .with_bearer(user_access_token)
        .body(body)
        .build()
        .map_err(|e| ShopError::from_error("malformed request", Box::new(e)))?;

    http::execute_checked(request).await?;
    Ok(())
}
