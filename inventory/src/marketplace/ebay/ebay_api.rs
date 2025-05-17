use super::client;
use crate::environment::RuntimeEnvironment;
use crate::listing::{listing_db, Listing, ListingEntity};
use crate::marketplace::ebay::client::{AuthorizationCodeResponse, ClientCredentialsResponse, RefreshTokenResponse};
use crate::marketplace::ebay::ebay_action;
use crate::{http, unwrap_option_else_404, unwrap_result_else_400, unwrap_result_else_500, ShopEntity};
use actix_web::cookie::Cookie;
use actix_web::http::StatusCode;
use actix_web::web::ServiceConfig;
use actix_web::{web, HttpRequest, HttpResponse, Responder};
use sqlx::PgPool;
use std::collections::BTreeMap;
use std::sync::LazyLock;
use uuid::Uuid;

static EBAY_OAUTH_AUTHORIZATION_URL: LazyLock<&str> = LazyLock::new(|| match RuntimeEnvironment::default() {
    RuntimeEnvironment::Local | RuntimeEnvironment::Stage => "https://auth.sandbox.ebay.com/oauth2/authorize?client_id=ZacharyS-shop-SBX-9a6e149a0-59597965&response_type=code&redirect_uri=Zachary_Siegel-ZacharyS-shop-S-kdujedb&scope=https://api.ebay.com/oauth/api_scope https://api.ebay.com/oauth/api_scope/buy.order.readonly https://api.ebay.com/oauth/api_scope/buy.guest.order https://api.ebay.com/oauth/api_scope/sell.marketing.readonly https://api.ebay.com/oauth/api_scope/sell.marketing https://api.ebay.com/oauth/api_scope/sell.inventory.readonly https://api.ebay.com/oauth/api_scope/sell.inventory https://api.ebay.com/oauth/api_scope/sell.account.readonly https://api.ebay.com/oauth/api_scope/sell.account https://api.ebay.com/oauth/api_scope/sell.fulfillment.readonly https://api.ebay.com/oauth/api_scope/sell.fulfillment https://api.ebay.com/oauth/api_scope/sell.analytics.readonly https://api.ebay.com/oauth/api_scope/sell.marketplace.insights.readonly https://api.ebay.com/oauth/api_scope/commerce.catalog.readonly https://api.ebay.com/oauth/api_scope/buy.shopping.cart https://api.ebay.com/oauth/api_scope/buy.offer.auction https://api.ebay.com/oauth/api_scope/commerce.identity.readonly https://api.ebay.com/oauth/api_scope/commerce.identity.email.readonly https://api.ebay.com/oauth/api_scope/commerce.identity.phone.readonly https://api.ebay.com/oauth/api_scope/commerce.identity.address.readonly https://api.ebay.com/oauth/api_scope/commerce.identity.name.readonly https://api.ebay.com/oauth/api_scope/commerce.identity.status.readonly https://api.ebay.com/oauth/api_scope/sell.finances https://api.ebay.com/oauth/api_scope/sell.payment.dispute https://api.ebay.com/oauth/api_scope/sell.item.draft https://api.ebay.com/oauth/api_scope/sell.item https://api.ebay.com/oauth/api_scope/sell.reputation https://api.ebay.com/oauth/api_scope/sell.reputation.readonly https://api.ebay.com/oauth/api_scope/commerce.notification.subscription https://api.ebay.com/oauth/api_scope/commerce.notification.subscription.readonly https://api.ebay.com/oauth/api_scope/sell.stores https://api.ebay.com/oauth/api_scope/sell.stores.readonly",
    RuntimeEnvironment::Production => "todo", // todo: update this and the testing url when we have an official eBay account.
});

const EBAY_USER_ACCESS_TOKEN_COOKIE_NAME: &str = "ebay_user_access_token";
const EBAY_USER_REFRESH_TOKEN_COOKIE_NAME: &str = "ebay_user_refresh_token";

pub fn configurer(config: &mut ServiceConfig) {
    config.service(
        web::scope("/ebay")
            .route("/auth/application/token", web::get().to(get_application_token))
            .route("/auth/user/token", web::get().to(get_user_token))
            .route("/auth/user/redirect", web::get().to(get_oauth_redirect))
            .route("/auth/user/refresh", web::put().to(refresh_user_token))
            .route("/listing/{listing_id}", web::put().to(put_listing))
    );
}

async fn get_application_token() -> impl Responder {
    let token_response: ClientCredentialsResponse = unwrap_result_else_500!(
        client::get_application_token().await
    );

    /* Don't set a cookie here. The client does not need it.
        If both application and user tokens are present, their sum is >4kB which triggers 431 from Actix.
        It seems Actix does not allow configuration of this 4kB maximum. */
    HttpResponse::Ok().json(token_response)
}

async fn get_user_token(
    query: web::Query<BTreeMap<String, String>>,
) -> impl Responder {
    let authorization_code: &String = unwrap_result_else_400!(query.get("code").ok_or("Missing authorization code"));
    let user_token_response: AuthorizationCodeResponse = match client::get_user_token(authorization_code).await {
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
    let refresh_token: Cookie = match request.cookie(EBAY_USER_REFRESH_TOKEN_COOKIE_NAME) {
        Some(value) => value,
        None => return HttpResponse::build(StatusCode::UNAUTHORIZED)
            .insert_header(("Location", EBAY_OAUTH_AUTHORIZATION_URL.to_string()))
            .body("Invalid eBay refresh token"),
    };

    let refresh_token_response: RefreshTokenResponse = match client::refresh_user_token(refresh_token.value()).await {
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

async fn put_listing(
    pgpool: web::Data<PgPool>,
    listing_id: web::Path<String>,
    request: HttpRequest,
) -> impl Responder {
    let user_token: Cookie = match request.cookie(EBAY_USER_ACCESS_TOKEN_COOKIE_NAME) {
        Some(value) => value,
        None => return HttpResponse::build(StatusCode::UNAUTHORIZED)
            .insert_header(("Location", EBAY_OAUTH_AUTHORIZATION_URL.to_string()))
            .body("Invalid eBay access token"),
    };

    let listing_id: Uuid = unwrap_result_else_400!(Uuid::try_parse(&listing_id));
    let listing: Option<ListingEntity> = unwrap_result_else_500!(
        listing_db::get_listing(&pgpool, &listing_id).await
    );
    let listing: ListingEntity = unwrap_option_else_404!(listing);
    let listing: Listing = unwrap_result_else_500!(listing.try_to_model());

    let _result = ebay_action::post(user_token.value(), &pgpool, &listing).await;
    HttpResponse::Gone().body("todo")
}
