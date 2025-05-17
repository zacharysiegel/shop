use crate::environment::RuntimeEnvironment;
use crate::marketplace::ebay::client::{AuthorizationCodeResponse, ClientCredentialsResponse};
use crate::{unwrap_result_else_400, unwrap_result_else_500};
use actix_web::web::ServiceConfig;
use actix_web::{web, HttpResponse, Responder};
use std::collections::BTreeMap;
use std::sync::LazyLock;

static EBAY_OAUTH_AUTHORIZATION_URL: LazyLock<&str> = LazyLock::new(|| match RuntimeEnvironment::default() {
    RuntimeEnvironment::Local | RuntimeEnvironment::Stage => "https://auth.sandbox.ebay.com/oauth2/authorize?client_id=ZacharyS-shop-SBX-9a6e149a0-59597965&response_type=code&redirect_uri=Zachary_Siegel-ZacharyS-shop-S-kdujedb&scope=https://api.ebay.com/oauth/api_scope https://api.ebay.com/oauth/api_scope/buy.order.readonly https://api.ebay.com/oauth/api_scope/buy.guest.order https://api.ebay.com/oauth/api_scope/sell.marketing.readonly https://api.ebay.com/oauth/api_scope/sell.marketing https://api.ebay.com/oauth/api_scope/sell.inventory.readonly https://api.ebay.com/oauth/api_scope/sell.inventory https://api.ebay.com/oauth/api_scope/sell.account.readonly https://api.ebay.com/oauth/api_scope/sell.account https://api.ebay.com/oauth/api_scope/sell.fulfillment.readonly https://api.ebay.com/oauth/api_scope/sell.fulfillment https://api.ebay.com/oauth/api_scope/sell.analytics.readonly https://api.ebay.com/oauth/api_scope/sell.marketplace.insights.readonly https://api.ebay.com/oauth/api_scope/commerce.catalog.readonly https://api.ebay.com/oauth/api_scope/buy.shopping.cart https://api.ebay.com/oauth/api_scope/buy.offer.auction https://api.ebay.com/oauth/api_scope/commerce.identity.readonly https://api.ebay.com/oauth/api_scope/commerce.identity.email.readonly https://api.ebay.com/oauth/api_scope/commerce.identity.phone.readonly https://api.ebay.com/oauth/api_scope/commerce.identity.address.readonly https://api.ebay.com/oauth/api_scope/commerce.identity.name.readonly https://api.ebay.com/oauth/api_scope/commerce.identity.status.readonly https://api.ebay.com/oauth/api_scope/sell.finances https://api.ebay.com/oauth/api_scope/sell.payment.dispute https://api.ebay.com/oauth/api_scope/sell.item.draft https://api.ebay.com/oauth/api_scope/sell.item https://api.ebay.com/oauth/api_scope/sell.reputation https://api.ebay.com/oauth/api_scope/sell.reputation.readonly https://api.ebay.com/oauth/api_scope/commerce.notification.subscription https://api.ebay.com/oauth/api_scope/commerce.notification.subscription.readonly https://api.ebay.com/oauth/api_scope/sell.stores https://api.ebay.com/oauth/api_scope/sell.stores.readonly",
    RuntimeEnvironment::Production => "todo", // todo: update this and the testing url when we have an official eBay account.
});

pub fn configurer(config: &mut ServiceConfig) {
    config.service(
        web::scope("/ebay")
            .route("/auth/application/token", web::get().to(get_application_token))
            .route("/auth/user/token", web::get().to(get_user_token))
            .route("/auth/user/redirect", web::get().to(get_oauth_redirect))
    );
}

async fn get_application_token() -> impl Responder {
    let token_response: ClientCredentialsResponse = unwrap_result_else_500!(
        super::client::get_application_token().await
    );
    HttpResponse::Ok().json(token_response)
}

async fn get_user_token(
    query: web::Query<BTreeMap<String, String>>,
) -> impl Responder {
    let authorization_code: &String = unwrap_result_else_400!(query.get("code").ok_or("Missing authorization code"));
    let user_token_response: AuthorizationCodeResponse = unwrap_result_else_500!(
        super::client::get_user_token(authorization_code).await
    );
    HttpResponse::Ok().json(user_token_response)
}

async fn get_oauth_redirect() -> impl Responder {
    HttpResponse::Found()
        .insert_header(("Location", EBAY_OAUTH_AUTHORIZATION_URL.to_string()))
        .finish()
}
