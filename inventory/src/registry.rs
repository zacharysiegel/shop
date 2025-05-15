use crate::environment::RuntimeEnvironment;
use reqwest::Client;
use std::sync::LazyLock;

pub struct Registry {
    pub http_client: Client,
    pub ebay_base_url: &'static str,
    pub ebay_client_id: &'static str,
}

pub static REGISTRY: LazyLock<Registry> = LazyLock::new(|| {
    let environment = RuntimeEnvironment::default();

    Registry {
        http_client: Client::builder().build().unwrap(),
        ebay_base_url: match environment {
            RuntimeEnvironment::Local | RuntimeEnvironment::Stage => "https://api.sandbox.ebay.com/",
            RuntimeEnvironment::Production => "https://api.ebay.com/"
        },
        // This value pertains to my testing "zach" account. Presumably this will change to an official eBay account.
        ebay_client_id: "ZacharyS-shop-SBX-9a6e149a0-59597965",
    }
});
