use crate::ebay::ebay_client::ebay_client_shared_xml::{EbayXmlHeader, EBAY_SCHEMA_VERSION};
use crate::ebay::ebay_client::{ebay_client_shared, ebay_client_shared_xml, ClientCredentialsResponse};
use crate::error::ShopError;
use crate::http;
use crate::http::{WithBearer, HTTP_CLIENT};
use crate::item_image::ItemImage;
use ebay_client_shared::{EBAY_BASE_URL, EBAY_CONTENT_LANGUAGE};
use ebay_client_shared_xml::{WarningLevel, EBAY_ERROR_LANGUAGE};
use reqwest::header::{CACHE_CONTROL, CONTENT_LANGUAGE};
use reqwest::multipart::Form;
use reqwest::{Request, Response};

const TRADING_API_BASE_PATH: &str = "/ws/api.dll";

enum PictureSetCodeType {
    Standard,
    Supersize,
}

impl PictureSetCodeType {
    pub fn get_serial_value(&self) -> &'static str {
        match self {
            PictureSetCodeType::Standard => "Standard",
            PictureSetCodeType::Supersize => "Supersize",
        }
    }
}

/// This eBay endpoint seems to be broken. We opt to serve images directly from our own server instead.
pub async fn upload_image(
    user_access_token: &str,
    item_image: &ItemImage,
) -> Result<String, ShopError> {
    let application_token: ClientCredentialsResponse = super::get_application_token().await?;

    let xml_payload: String = format!(
        r#"
        <?xml version="1.0" encoding="utf-8"?>
        <UploadSiteHostedPicturesRequest xmlns="urn:ebay:apis:eBLBaseComponents">
            <!-- see X-EBAY-API-IAF-TOKEN, todo: remove if header works
            <RequesterCredentials>
                <ebl:eBayAuthToken xmlns:ebl="urn:ebay:apis:eBLBaseComponents">{}</ebl:eBayAuthToken>
            </RequesterCredentials>
            -->
            <!-- Call-specific Input Fields -->
            <!-- <ExternalPictureURL> anyURI </ExternalPictureURL> -->
            <PictureName>{}</PictureName>
            <PictureSet>{}</PictureSet>
            <PictureSystemVersion>2</PictureSystemVersion> <!-- "Only version 2 is valid" 07/01/2025 -->
            <PictureUploadPolicy>Add</PictureUploadPolicy>
            <!-- Standard Input Fields -->
            <ErrorLanguage>{}</ErrorLanguage>
            <MessageID>{1}</MessageID>
            <WarningLevel>{}</WarningLevel>
        </UploadSiteHostedPicturesRequest>
    "#,
        application_token.access_token,
        item_image.id,
        PictureSetCodeType::Standard.get_serial_value(),
        EBAY_ERROR_LANGUAGE,
        WarningLevel::High.get_serial_value(),
    );
    let body: Form = Form::new()
        .text("XML Payload", xml_payload)
        .file("file", item_image.get_item_image_path()?).await
        .map_err(|e| ShopError::from_error_default(Box::new(e)))?;

    let request: Request = HTTP_CLIENT
        .put(format!("{}{}", *EBAY_BASE_URL, TRADING_API_BASE_PATH))
        .header(CONTENT_LANGUAGE, EBAY_CONTENT_LANGUAGE)
        .header(CACHE_CONTROL, "no-cache")
        .header(EbayXmlHeader::XEbayApiIafToken.to_serial_value(), user_access_token)
        .header(EbayXmlHeader::XEbayApiCallName.to_serial_value(), "UploadSiteHostedPictures")
        .header(EbayXmlHeader::XEbayApiSiteId.to_serial_value(), "0")
        // todo: needed?
        // .header("X-EBAY-API-RESPONSE-ENCODING", "XML")
        .header(EbayXmlHeader::XEbayApiCompatibilityLevel.to_serial_value(), EBAY_SCHEMA_VERSION)
        // todo: needed?
        // .header("X-EBAY-API-DETAIL-LEVEL", "0")
        .with_bearer(user_access_token)
        .multipart(body)
        .build()
        .map_err(|e| ShopError::from_error("malformed request", Box::new(e)))?;
    log::info!("request: {:?}", request);

    let response: Response = http::execute_checked(request).await?;
    let text = response.text().await
        .map_err(|e| ShopError::from_error_default(Box::new(e)))?;
    log::info!("response text: {}", text);

    Ok(text)
}
