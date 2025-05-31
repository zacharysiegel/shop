use crate::error::ShopError;
use crate::http;
use crate::marketplace::ebay::ebay_client::ebay_client_shared::{ebay_basic_auth, EBAY_BASE_URL};
use reqwest::header::{AUTHORIZATION, CONTENT_TYPE};
use reqwest::{Request, Response};
use serde::{Deserialize, Serialize};

const EBAY_REDIRECT_URL_NAME: &str = "Zachary_Siegel-ZacharyS-shop-S-kdujedb";
/// https://developer.ebay.com/api-docs/sell/identity/overview.html
const OAUTH_API_BASE_PATH: &str = "/identity/v1/oauth2";
const APPLICATION_SCOPE_ALL: &str = "https://api.ebay.com/oauth/api_scope+https://api.ebay.com/oauth/api_scope/buy.guest.order+https://api.ebay.com/oauth/api_scope/buy.item.feed+https://api.ebay.com/oauth/api_scope/buy.marketing+https://api.ebay.com/oauth/api_scope/buy.product.feed+https://api.ebay.com/oauth/api_scope/buy.marketplace.insights+https://api.ebay.com/oauth/api_scope/buy.proxy.guest.order+https://api.ebay.com/oauth/api_scope/buy.item.bulk+https://api.ebay.com/oauth/api_scope/buy.deal";
const USER_SCOPE_ALL: &str = "https://api.ebay.com/oauth/api_scope+https://api.ebay.com/oauth/api_scope/buy.order.readonly+https://api.ebay.com/oauth/api_scope/buy.guest.order+https://api.ebay.com/oauth/api_scope/sell.marketing.readonly+https://api.ebay.com/oauth/api_scope/sell.marketing+https://api.ebay.com/oauth/api_scope/sell.inventory.readonly+https://api.ebay.com/oauth/api_scope/sell.inventory+https://api.ebay.com/oauth/api_scope/sell.account.readonly+https://api.ebay.com/oauth/api_scope/sell.account+https://api.ebay.com/oauth/api_scope/sell.fulfillment.readonly+https://api.ebay.com/oauth/api_scope/sell.fulfillment+https://api.ebay.com/oauth/api_scope/sell.analytics.readonly+https://api.ebay.com/oauth/api_scope/sell.marketplace.insights.readonly+https://api.ebay.com/oauth/api_scope/commerce.catalog.readonly+https://api.ebay.com/oauth/api_scope/buy.shopping.cart+https://api.ebay.com/oauth/api_scope/buy.offer.auction+https://api.ebay.com/oauth/api_scope/commerce.identity.readonly+https://api.ebay.com/oauth/api_scope/commerce.identity.email.readonly+https://api.ebay.com/oauth/api_scope/commerce.identity.phone.readonly+https://api.ebay.com/oauth/api_scope/commerce.identity.address.readonly+https://api.ebay.com/oauth/api_scope/commerce.identity.name.readonly+https://api.ebay.com/oauth/api_scope/commerce.identity.status.readonly+https://api.ebay.com/oauth/api_scope/sell.finances+https://api.ebay.com/oauth/api_scope/sell.payment.dispute+https://api.ebay.com/oauth/api_scope/sell.item.draft+https://api.ebay.com/oauth/api_scope/sell.item+https://api.ebay.com/oauth/api_scope/sell.reputation+https://api.ebay.com/oauth/api_scope/sell.reputation.readonly+https://api.ebay.com/oauth/api_scope/commerce.notification.subscription+https://api.ebay.com/oauth/api_scope/commerce.notification.subscription.readonly+https://api.ebay.com/oauth/api_scope/sell.stores+https://api.ebay.com/oauth/api_scope/sell.stores.readonly";

#[derive(Serialize, Deserialize)]
pub struct ClientCredentialsResponse {
    pub access_token: String,
    /// Seconds
    pub expires_in: u64,
    pub token_type: String,
}

pub async fn get_application_token() -> Result<ClientCredentialsResponse, ShopError> {
    let request: Request = http::HTTP_CLIENT
        .post(format!("{}{}/token", *EBAY_BASE_URL, OAUTH_API_BASE_PATH))
        .header(CONTENT_TYPE, "application/x-www-form-urlencoded")
        .header(AUTHORIZATION, format!("Basic {}", ebay_basic_auth()))
        .body(format!("grant_type=client_credentials&scope={}", APPLICATION_SCOPE_ALL))
        .build()
        .map_err(|error| ShopError::from_error("malformed request", Box::new(error)))?;

    let response: Response = http::execute_checked(request).await?;
    let body: ClientCredentialsResponse = response.json::<ClientCredentialsResponse>().await
        .map_err(|error| ShopError::from_error("parsing response", Box::new(error)))?;

    Ok(body)
}

#[derive(Serialize, Deserialize)]
pub struct AuthorizationCodeResponse {
    pub access_token: String,
    /// Seconds
    pub expires_in: u64,
    pub refresh_token: String,
    /// Seconds
    pub refresh_token_expires_in: u64,
    pub token_type: String,
}

pub async fn get_user_token(authorization_code: &str) -> Result<AuthorizationCodeResponse, ShopError> {
    let request: Request = http::HTTP_CLIENT
        .post(format!("{}{}/token", *EBAY_BASE_URL, OAUTH_API_BASE_PATH))
        .header(CONTENT_TYPE, "application/x-www-form-urlencoded")
        .header(AUTHORIZATION, format!("Basic {}", ebay_basic_auth()))
        .body(format!("grant_type=authorization_code&redirect_uri={}&code={}", EBAY_REDIRECT_URL_NAME, authorization_code))
        .build()
        .map_err(|error| ShopError::from_error("malformed request", Box::new(error)))?;

    let response: Response = http::execute_checked(request).await?;
    let body: AuthorizationCodeResponse = response.json::<AuthorizationCodeResponse>().await
        .map_err(|error| ShopError::from_error("parsing response", Box::new(error)))?;

    Ok(body)
}

#[derive(Serialize, Deserialize)]
pub struct RefreshTokenResponse {
    pub access_token: String,
    /// Seconds
    pub expires_in: u64,
    pub token_type: String,
}

pub async fn refresh_user_token(refresh_token: &str) -> Result<RefreshTokenResponse, ShopError> {
    let request: Request = http::HTTP_CLIENT
        .post(format!("{}{}/token", *EBAY_BASE_URL, OAUTH_API_BASE_PATH))
        .header(CONTENT_TYPE, "application/x-www-form-urlencoded")
        .header(AUTHORIZATION, format!("Basic {}", ebay_basic_auth()))
        .body(format!("grant_type=refresh_token&refresh_token={}&scope={}", refresh_token, USER_SCOPE_ALL))
        .build()
        .map_err(|error| ShopError::from_error("malformed request", Box::new(error)))?;

    let response: Response = http::execute_checked(request).await?;
    let body: RefreshTokenResponse = response.json::<RefreshTokenResponse>().await
        .map_err(|error| ShopError::from_error("parsing response", Box::new(error)))?;

    Ok(body)
}
