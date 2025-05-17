use crate::decrypt::master_decrypt;
use crate::environment::RuntimeEnvironment;
use crate::http::BASE64;
use base64::Engine;
use std::sync::LazyLock;

pub static EBAY_BASE_URL: LazyLock<&str> = LazyLock::new(|| match RuntimeEnvironment::default() {
    RuntimeEnvironment::Local | RuntimeEnvironment::Stage => "https://api.sandbox.ebay.com",
    RuntimeEnvironment::Production => "https://api.ebay.com"
});

// This value pertains to my testing "zach" account. Presumably this will change to an official eBay account.
// This value pertains to my testing "zach" account. Presumably this will change to an official eBay account.
/// Presented as a UTF-8-encoded string because this secret must be re-encoded with the client ID in base64 to form the basic authentication credential
pub static EBAY_CLIENT_SECRET: LazyLock<String> = LazyLock::new(||
    String::from_utf8(master_decrypt("ebay__zach.sandbox.cert_id").unwrap())
        .unwrap()
);

pub const EBAY_CLIENT_ID: &str = "ZacharyS-shop-SBX-9a6e149a0-59597965";

pub const EBAY_MARKETPLACE_US: &str = "EBAY_US";

// https://developer.ebay.com/api-docs/sell/account/types/ba:MarketplaceIdEnum
pub const EBAY_CATEGORY_STANDARD: &str = "ALL_EXCLUDING_MOTORS_VEHICLES";

// https://developer.ebay.com/api-docs/sell/account/types/api:CategoryTypeEnum
pub const EBAY_CONTENT_LANGUAGE: &str = "en-US";


/// Returns the base64-encoded basic authentication value.
pub fn ebay_basic_auth() -> String {
    let raw: String = format!("{}:{}", EBAY_CLIENT_ID, *EBAY_CLIENT_SECRET);
    BASE64.encode(raw.as_bytes())
}
