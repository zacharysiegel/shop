use crate::decrypt::master_decrypt;
use crate::environment::RuntimeEnvironment;
use crate::error::ShopError;
use crate::item::Item;
use crate::listing::Listing;
use crate::product::Product;
use crate::registry::{BASE64, REGISTRY};
use base64::Engine;
use reqwest::header::{AUTHORIZATION, CONTENT_TYPE};
use reqwest::Request;
use std::sync::LazyLock;
use serde::Deserialize;

static EBAY_BASE_URL: LazyLock<&str> = LazyLock::new(|| match RuntimeEnvironment::default() {
    RuntimeEnvironment::Local | RuntimeEnvironment::Stage => "https://api.sandbox.ebay.com",
    RuntimeEnvironment::Production => "https://api.ebay.com"
});
// This value pertains to my testing "zach" account. Presumably this will change to an official eBay account.
// This value pertains to my testing "zach" account. Presumably this will change to an official eBay account.
/// Presented as a UTF-8-encoded string because this secret must be re-encoded with the client ID in base64 to form the basic authentication credential
static EBAY_CLIENT_SECRET: LazyLock<String> = LazyLock::new(|| String::from_utf8(master_decrypt("ebay__zach.sandbox.cert_id").unwrap()).unwrap());
static EBAY_OAUTH_AUTHORIZATION_URL: LazyLock<&str> = LazyLock::new(|| match RuntimeEnvironment::default() {
    RuntimeEnvironment::Local | RuntimeEnvironment::Stage => "https://auth.sandbox.ebay.com/oauth2/authorize?client_id=ZacharyS-shop-SBX-9a6e149a0-59597965&response_type=code&redirect_uri=Zachary_Siegel-ZacharyS-shop-S-kdujedb&scope=https://api.ebay.com/oauth/api_scope https://api.ebay.com/oauth/api_scope/buy.order.readonly https://api.ebay.com/oauth/api_scope/buy.guest.order https://api.ebay.com/oauth/api_scope/sell.marketing.readonly https://api.ebay.com/oauth/api_scope/sell.marketing https://api.ebay.com/oauth/api_scope/sell.inventory.readonly https://api.ebay.com/oauth/api_scope/sell.inventory https://api.ebay.com/oauth/api_scope/sell.account.readonly https://api.ebay.com/oauth/api_scope/sell.account https://api.ebay.com/oauth/api_scope/sell.fulfillment.readonly https://api.ebay.com/oauth/api_scope/sell.fulfillment https://api.ebay.com/oauth/api_scope/sell.analytics.readonly https://api.ebay.com/oauth/api_scope/sell.marketplace.insights.readonly https://api.ebay.com/oauth/api_scope/commerce.catalog.readonly https://api.ebay.com/oauth/api_scope/buy.shopping.cart https://api.ebay.com/oauth/api_scope/buy.offer.auction https://api.ebay.com/oauth/api_scope/commerce.identity.readonly https://api.ebay.com/oauth/api_scope/commerce.identity.email.readonly https://api.ebay.com/oauth/api_scope/commerce.identity.phone.readonly https://api.ebay.com/oauth/api_scope/commerce.identity.address.readonly https://api.ebay.com/oauth/api_scope/commerce.identity.name.readonly https://api.ebay.com/oauth/api_scope/commerce.identity.status.readonly https://api.ebay.com/oauth/api_scope/sell.finances https://api.ebay.com/oauth/api_scope/sell.payment.dispute https://api.ebay.com/oauth/api_scope/sell.item.draft https://api.ebay.com/oauth/api_scope/sell.item https://api.ebay.com/oauth/api_scope/sell.reputation https://api.ebay.com/oauth/api_scope/sell.reputation.readonly https://api.ebay.com/oauth/api_scope/commerce.notification.subscription https://api.ebay.com/oauth/api_scope/commerce.notification.subscription.readonly https://api.ebay.com/oauth/api_scope/sell.stores https://api.ebay.com/oauth/api_scope/sell.stores.readonly",
    RuntimeEnvironment::Production => "todo", // todo: update this and the testing url when we have an official eBay account.
});

const EBAY_CLIENT_ID: &str = "ZacharyS-shop-SBX-9a6e149a0-59597965";
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

#[derive(Deserialize)]
struct ClientCredentialsResponse {
    access_token: String,
    expires_in: u64,
    token_type: String,
}

pub(super) async fn get_application_token() -> Result<String, ShopError> {
    let request: Request = REGISTRY.http_client
        .post(format!("{}{}/token", *EBAY_BASE_URL, OAUTH_API_BASE_PATH))
        .header(CONTENT_TYPE, "x-www-form-urlencoded")
        .header(AUTHORIZATION, format!("Basic {}", basic_auth()))
        .body("grant_type=client_credentials&scope=https://api.ebay.com/oauth/api_scope+https://api.ebay.com/oauth/api_scope/buy.guest.order+https://api.ebay.com/oauth/api_scope/buy.item.feed+https://api.ebay.com/oauth/api_scope/buy.marketing+https://api.ebay.com/oauth/api_scope/buy.product.feed+https://api.ebay.com/oauth/api_scope/buy.marketplace.insights+https://api.ebay.com/oauth/api_scope/buy.proxy.guest.order+https://api.ebay.com/oauth/api_scope/buy.item.bulk+https://api.ebay.com/oauth/api_scope/buy.deal")
        .build()
        .map_err(|error| ShopError::from_error("malformed request", Box::new(error)))?;

    let response = REGISTRY.http_client.execute(request)
        .await
        .map_err(|error| ShopError::from_error("request failed", Box::new(error)))?;

    if response.status().is_client_error() || response.status().is_server_error() {
        let status = response.status();
        let text = response.text().await
            .map_err(|error| ShopError::from_error("reading http response", Box::new(error)))?;
        return Err(ShopError::new(&format!("http error; [{}]; [{}];", status, text)));
    }

    let client_credentials_response: ClientCredentialsResponse = response.json::<ClientCredentialsResponse>().await
        .map_err(|error| ShopError::from_error("parsing response", Box::new(error)))?;

    Ok(client_credentials_response.access_token)
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
