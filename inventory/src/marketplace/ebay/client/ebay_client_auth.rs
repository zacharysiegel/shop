use crate::error::ShopError;
use crate::http;
use crate::marketplace::ebay::client::ebay_client_shared::{ebay_basic_auth, EBAY_BASE_URL};
use reqwest::header::{AUTHORIZATION, CONTENT_TYPE};
use reqwest::{Request, Response};
use serde::{Deserialize, Serialize};

const EBAY_REDIRECT_URL_NAME: &str = "Zachary_Siegel-ZacharyS-shop-S-kdujedb";
/// https://developer.ebay.com/api-docs/sell/identity/overview.html
const OAUTH_API_BASE_PATH: &str = "/identity/v1/oauth2";

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
        .header(CONTENT_TYPE, "x-www-form-urlencoded")
        .header(AUTHORIZATION, format!("Basic {}", ebay_basic_auth()))
        .body("grant_type=client_credentials&scope=https://api.ebay.com/oauth/api_scope+https://api.ebay.com/oauth/api_scope/buy.guest.order+https://api.ebay.com/oauth/api_scope/buy.item.feed+https://api.ebay.com/oauth/api_scope/buy.marketing+https://api.ebay.com/oauth/api_scope/buy.product.feed+https://api.ebay.com/oauth/api_scope/buy.marketplace.insights+https://api.ebay.com/oauth/api_scope/buy.proxy.guest.order+https://api.ebay.com/oauth/api_scope/buy.item.bulk+https://api.ebay.com/oauth/api_scope/buy.deal")
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
        .header(CONTENT_TYPE, "x-www-form-urlencoded")
        .header(AUTHORIZATION, format!("Basic {}", ebay_basic_auth()))
        .body(format!("grant_type=authorization_code&redirect_uri={}&code={}", EBAY_REDIRECT_URL_NAME, authorization_code))
        .build()
        .map_err(|error| ShopError::from_error("malformed request", Box::new(error)))?;

    let response: Response = http::execute_checked(request).await?;
    let body: AuthorizationCodeResponse = response.json::<AuthorizationCodeResponse>().await
        .map_err(|error| ShopError::from_error("parsing response", Box::new(error)))?;

    Ok(body)
}
