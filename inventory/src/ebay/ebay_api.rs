use super::ebay_client;
use crate::ebay::ebay_action;
use crate::ebay::ebay_client::{AuthorizationCodeResponse, ClientCredentialsResponse, RefreshTokenResponse};
use crate::environment::RuntimeEnvironment;
use crate::listing::{listing_db, Listing, ListingEntity, ListingStatus};
use crate::{http, unwrap_option_else_400, unwrap_option_else_404, unwrap_result_else_400, unwrap_result_else_500, ShopEntity};
use actix_web::cookie::Cookie;
use actix_web::http::StatusCode;
use actix_web::web::ServiceConfig;
use actix_web::{web, HttpRequest, HttpResponse, Responder};
use serde::Deserialize;
use serde_json::{Map, Value};
use sqlx::PgPool;
use std::sync::LazyLock;
use uuid::Uuid;

pub static EBAY_OAUTH_AUTHORIZATION_URL: LazyLock<&str> = LazyLock::new(|| match RuntimeEnvironment::default() {
    RuntimeEnvironment::Local | RuntimeEnvironment::Stage => "https://auth.sandbox.ebay.com/oauth2/authorize?client_id=ZacharyS-shop-SBX-9a6e149a0-59597965&response_type=code&redirect_uri=Zachary_Siegel-ZacharyS-shop-S-kdujedb&scope=https://api.ebay.com/oauth/api_scope https://api.ebay.com/oauth/api_scope/buy.order.readonly https://api.ebay.com/oauth/api_scope/buy.guest.order https://api.ebay.com/oauth/api_scope/sell.marketing.readonly https://api.ebay.com/oauth/api_scope/sell.marketing https://api.ebay.com/oauth/api_scope/sell.inventory.readonly https://api.ebay.com/oauth/api_scope/sell.inventory https://api.ebay.com/oauth/api_scope/sell.account.readonly https://api.ebay.com/oauth/api_scope/sell.account https://api.ebay.com/oauth/api_scope/sell.fulfillment.readonly https://api.ebay.com/oauth/api_scope/sell.fulfillment https://api.ebay.com/oauth/api_scope/sell.analytics.readonly https://api.ebay.com/oauth/api_scope/sell.marketplace.insights.readonly https://api.ebay.com/oauth/api_scope/commerce.catalog.readonly https://api.ebay.com/oauth/api_scope/buy.shopping.cart https://api.ebay.com/oauth/api_scope/buy.offer.auction https://api.ebay.com/oauth/api_scope/commerce.identity.readonly https://api.ebay.com/oauth/api_scope/commerce.identity.email.readonly https://api.ebay.com/oauth/api_scope/commerce.identity.phone.readonly https://api.ebay.com/oauth/api_scope/commerce.identity.address.readonly https://api.ebay.com/oauth/api_scope/commerce.identity.name.readonly https://api.ebay.com/oauth/api_scope/commerce.identity.status.readonly https://api.ebay.com/oauth/api_scope/sell.finances https://api.ebay.com/oauth/api_scope/sell.payment.dispute https://api.ebay.com/oauth/api_scope/sell.item.draft https://api.ebay.com/oauth/api_scope/sell.item https://api.ebay.com/oauth/api_scope/sell.reputation https://api.ebay.com/oauth/api_scope/sell.reputation.readonly https://api.ebay.com/oauth/api_scope/commerce.notification.subscription https://api.ebay.com/oauth/api_scope/commerce.notification.subscription.readonly https://api.ebay.com/oauth/api_scope/sell.stores https://api.ebay.com/oauth/api_scope/sell.stores.readonly",
    RuntimeEnvironment::Production => "todo", // todo: update this and the testing url when we have an official eBay account.
});

const EBAY_USER_ACCESS_TOKEN_COOKIE_NAME: &str = "ebay_user_access_token";
const EBAY_USER_REFRESH_TOKEN_COOKIE_NAME: &str = "ebay_user_refresh_token";

pub fn configurer(config: &mut ServiceConfig) {
    config.service(
        web::scope("/ebay")
            .route("/auth/application/token", web::get().to(get_application_token))
            .route("/auth/user/token", web::put().to(get_user_token))
            .route("/auth/user/redirect", web::get().to(get_oauth_redirect))
            .route("/auth/user/refresh", web::put().to(refresh_user_token))
            .route("/listing/{item_id}", web::put().to(publish_listing))
            .route("/listing", web::put().to(publish_all_listings))
            .route("/listing/{item_id}", web::get().to(get_listing))
            .route("/location", web::get().to(get_all_locations))
            .route("/location", web::put().to(sync_locations))
    );
}

fn extract_user_token(request: &HttpRequest) -> Result<Cookie, HttpResponse> {
    match request.cookie(EBAY_USER_ACCESS_TOKEN_COOKIE_NAME) {
        Some(value) => Ok(value),
        None => Err(HttpResponse::build(StatusCode::UNAUTHORIZED)
            .insert_header(("Location", EBAY_OAUTH_AUTHORIZATION_URL.to_string()))
            .insert_header(("Content-Type", "text/plain"))
            .body("Invalid eBay access token")),
    }
}

async fn get_application_token() -> impl Responder {
    let token_response: ClientCredentialsResponse = unwrap_result_else_500!(
        ebay_client::get_application_token().await
    );

    /* Don't set a cookie here. The client does not need it.
        If both application and user tokens are present, their sum is >4kB which triggers 431 from Actix.
        It seems Actix does not allow configuration of this 4kB maximum. */
    HttpResponse::Ok().json(token_response)
}

async fn get_user_token(
    body: String,
) -> impl Responder {
    let body: Map<String, Value> = unwrap_result_else_400!(serde_json::from_str::<Map<String, Value>>(body.as_str()));
    let authorization_code: &Value = unwrap_option_else_400!(body.get("code"));
    let Value::String(authorization_code) = authorization_code else {
        let message: &str = "Error deserializing JSON for user authorization code";
        log::error!("{}", message);
        return HttpResponse::BadRequest().body(message);
    };

    let user_token_response: AuthorizationCodeResponse = match ebay_client::get_user_token(authorization_code).await {
        Ok(value) => value,
        Err(error) => return HttpResponse::InternalServerError().body(error.to_string()),
    };

    HttpResponse::Ok()
        .append_header(http::header_set_cookie_secure(EBAY_USER_ACCESS_TOKEN_COOKIE_NAME, &user_token_response.access_token, user_token_response.expires_in))
        .append_header(http::header_set_cookie_secure(EBAY_USER_REFRESH_TOKEN_COOKIE_NAME, &user_token_response.refresh_token, user_token_response.expires_in))
        .json(user_token_response)
}

async fn refresh_user_token(
    request: HttpRequest,
) -> impl Responder {
    let user_refresh_token: Cookie = match request.cookie(EBAY_USER_REFRESH_TOKEN_COOKIE_NAME) {
        Some(value) => value,
        None => return HttpResponse::build(StatusCode::UNAUTHORIZED)
            .insert_header(("Location", EBAY_OAUTH_AUTHORIZATION_URL.to_string()))
            .body("Invalid eBay refresh token"),
    };

    let refresh_token_response: RefreshTokenResponse = match ebay_client::refresh_user_token(user_refresh_token.value()).await {
        Ok(value) => value,
        Err(error) => return HttpResponse::InternalServerError().body(error.to_string()),
    };

    HttpResponse::Ok()
        .append_header(http::header_set_cookie_secure(EBAY_USER_ACCESS_TOKEN_COOKIE_NAME, &refresh_token_response.access_token, refresh_token_response.expires_in))
        .json(refresh_token_response)
}

async fn get_oauth_redirect() -> impl Responder {
    HttpResponse::Found()
        .insert_header(("Location", EBAY_OAUTH_AUTHORIZATION_URL.to_string()))
        .finish()
}

async fn publish_listing(
    pgpool: web::Data<PgPool>,
    item_id: web::Path<String>,
    request: HttpRequest,
) -> impl Responder {
    let user_access_token: Cookie = match extract_user_token(&request) {
        Ok(value) => value,
        Err(response) => return response,
    };

    let listing_id: Uuid = unwrap_result_else_400!(Uuid::try_parse(&item_id));
    let listing: Option<ListingEntity> = unwrap_result_else_500!(
        listing_db::get_listing(&pgpool, &listing_id).await
    );
    let listing: ListingEntity = unwrap_option_else_404!(listing);
    let listing: Listing = unwrap_result_else_500!(listing.try_to_model());

    unwrap_result_else_500!(ebay_action::publish(&pgpool, &user_access_token.value(), &listing).await);
    HttpResponse::NoContent().finish()
}

#[derive(Deserialize)]
struct PublishAllListingsParameters {
    status: u8,
}

async fn publish_all_listings(
    pgpool: web::Data<PgPool>,
    request: HttpRequest,
    parameters: web::Query<PublishAllListingsParameters>,
) -> HttpResponse {
    let user_access_token: Cookie = match extract_user_token(&request) {
        Ok(value) => value,
        Err(response) => return response,
    };

    let status: ListingStatus = unwrap_option_else_400!(ListingStatus::from_repr(parameters.status));
    if (status != ListingStatus::Draft) {
        return HttpResponse::BadRequest().finish();
    }

    unwrap_result_else_500!(ebay_action::publish_all_with_status(&pgpool, user_access_token.value(), &status).await);
    HttpResponse::NoContent().finish()
}

async fn get_listing(
    item_id: web::Path<String>,
    request: HttpRequest,
) -> impl Responder {
    let user_access_token: Cookie = match extract_user_token(&request) {
        Ok(value) => value,
        Err(response) => return response,
    };

    let json: Value = unwrap_result_else_500!(ebay_client::get_inventory_item(&user_access_token.value(), &item_id).await);
    HttpResponse::Ok().json(json)
}

async fn get_all_locations(
    request: HttpRequest,
) -> impl Responder {
    let user_access_token: Cookie = match extract_user_token(&request) {
        Ok(value) => value,
        Err(response) => return response,
    };

    let json: Value = unwrap_result_else_500!(ebay_client::get_all_inventory_locations(&user_access_token.value()).await);
    HttpResponse::Ok().json(json)
}

async fn sync_locations(
    pgpool: web::Data<PgPool>,
    request: HttpRequest,
) -> HttpResponse {
    let user_access_token: Cookie = match extract_user_token(&request) {
        Ok(value) => value,
        Err(response) => return response,
    };

    unwrap_result_else_500!(ebay_action::sync_all_locations(&pgpool, &user_access_token.value()).await);
    HttpResponse::build(StatusCode::NO_CONTENT).finish()
}
