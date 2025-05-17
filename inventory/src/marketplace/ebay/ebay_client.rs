use crate::decrypt::master_decrypt;
use crate::environment::RuntimeEnvironment;
use crate::error::ShopError;
use crate::http;
use crate::http::BASE64;
use crate::item::Item;
use crate::listing::Listing;
use crate::product::Product;
use base64::Engine;
use reqwest::header::{AUTHORIZATION, CONTENT_TYPE};
use reqwest::{Request, Response};
use serde::{Deserialize, Serialize};
use std::sync::LazyLock;

static EBAY_BASE_URL: LazyLock<&str> = LazyLock::new(|| match RuntimeEnvironment::default() {
    RuntimeEnvironment::Local | RuntimeEnvironment::Stage => "https://api.sandbox.ebay.com",
    RuntimeEnvironment::Production => "https://api.ebay.com"
});
// This value pertains to my testing "zach" account. Presumably this will change to an official eBay account.
// This value pertains to my testing "zach" account. Presumably this will change to an official eBay account.
/// Presented as a UTF-8-encoded string because this secret must be re-encoded with the client ID in base64 to form the basic authentication credential
static EBAY_CLIENT_SECRET: LazyLock<String> = LazyLock::new(|| String::from_utf8(master_decrypt("ebay__zach.sandbox.cert_id").unwrap()).unwrap());

const EBAY_CLIENT_ID: &str = "ZacharyS-shop-SBX-9a6e149a0-59597965";
const EBAY_REDIRECT_URL_NAME: &str = "Zachary_Siegel-ZacharyS-shop-S-kdujedb";
const EBAY_MARKETPLACE_US: &str = "EBAY_US";
// https://developer.ebay.com/api-docs/sell/account/types/ba:MarketplaceIdEnum
const EBAY_CATEGORY_STANDARD: &str = "ALL_EXCLUDING_MOTORS_VEHICLES";
// https://developer.ebay.com/api-docs/sell/account/types/api:CategoryTypeEnum
const EBAY_CONTENT_LANGUAGE: &str = "en-US";

const INVENTORY_API_BASE_PATH: &str = "/sell/inventory/v1";
/// https://developer.ebay.com/api-docs/sell/identity/overview.html
const OAUTH_API_BASE_PATH: &str = "/identity/v1/oauth2";

/// Returns the base64-encoded basic authentication value.
fn basic_auth() -> String {
    let raw: String = format!("{}:{}", EBAY_CLIENT_ID, *EBAY_CLIENT_SECRET);
    BASE64.encode(raw.as_bytes())
}

#[derive(Serialize, Deserialize)]
pub(super) struct ClientCredentialsResponse {
    access_token: String,
    expires_in: u64,
    token_type: String,
}

pub(super) async fn get_application_token() -> Result<ClientCredentialsResponse, ShopError> {
    let request: Request = http::HTTP_CLIENT
        .post(format!("{}{}/token", *EBAY_BASE_URL, OAUTH_API_BASE_PATH))
        .header(CONTENT_TYPE, "x-www-form-urlencoded")
        .header(AUTHORIZATION, format!("Basic {}", basic_auth()))
        .body("grant_type=client_credentials&scope=https://api.ebay.com/oauth/api_scope+https://api.ebay.com/oauth/api_scope/buy.guest.order+https://api.ebay.com/oauth/api_scope/buy.item.feed+https://api.ebay.com/oauth/api_scope/buy.marketing+https://api.ebay.com/oauth/api_scope/buy.product.feed+https://api.ebay.com/oauth/api_scope/buy.marketplace.insights+https://api.ebay.com/oauth/api_scope/buy.proxy.guest.order+https://api.ebay.com/oauth/api_scope/buy.item.bulk+https://api.ebay.com/oauth/api_scope/buy.deal")
        .build()
        .map_err(|error| ShopError::from_error("malformed request", Box::new(error)))?;

    let response: Response = http::execute_checked(request).await?;
    let body: ClientCredentialsResponse = response.json::<ClientCredentialsResponse>().await
        .map_err(|error| ShopError::from_error("parsing response", Box::new(error)))?;

    Ok(body)
}

#[derive(Serialize, Deserialize)]
pub(super) struct AuthorizationCodeResponse {
    access_token: String,
    expires_in: u64,
    refresh_token: String,
    refresh_token_expires_in: u64,
    token_type: String,
}

pub(super) async fn get_user_token(authorization_code: &str) -> Result<AuthorizationCodeResponse, ShopError> {
    let request: Request = http::HTTP_CLIENT
        .post(format!("{}{}/token", *EBAY_BASE_URL, OAUTH_API_BASE_PATH))
        .header(CONTENT_TYPE, "x-www-form-urlencoded")
        .header(AUTHORIZATION, format!("Basic {}", basic_auth()))
        .body(format!("grant_type=authorization_code&redirect_uri={}&code={}", EBAY_REDIRECT_URL_NAME, authorization_code))
        .build()
        .map_err(|error| ShopError::from_error("malformed request", Box::new(error)))?;

    let response: Response = http::execute_checked(request).await?;
    let body: AuthorizationCodeResponse = response.json::<AuthorizationCodeResponse>().await
        .map_err(|error| ShopError::from_error("parsing response", Box::new(error)))?;

    Ok(body)
}

pub(super) async fn create_listing(
    listing: &Listing,
    item: &Item,
    product: &Product,
) -> Result<(), ShopError> {
    Ok(()) // todo
}

pub(super) async fn publish_listing(
    listing: &Listing,
    item: &Item,
    product: &Product,
) -> Result<(), ShopError> {
    Ok(()) // todo
}
