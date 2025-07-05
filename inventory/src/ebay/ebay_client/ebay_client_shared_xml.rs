
pub const EBAY_ERROR_LANGUAGE: &str = "en_US";

/// Used in conjunction with X-EBAY-API-COMPATIBILITY-LEVEL
pub const EBAY_SCHEMA_VERSION: &str = "1415";

/// https://developer.ebay.com/api-docs/user-guides/static/make-a-call/using-xml.html#headers
#[allow(unused)]
pub enum EbayXmlHeader {
    XEbayApiIafToken,
    XEbayApiCompatibilityLevel,
    XEbayApiDevName,
    XEbayApiAppName,
    XEbayApiCertName,
    XEbayApiCallName,
    XEbayApiSiteId,
}

impl EbayXmlHeader {
    pub fn to_serial_value(&self) -> &'static str {
        match self {
            EbayXmlHeader::XEbayApiIafToken => "X-EBAY-IAF-TOKEN",
            EbayXmlHeader::XEbayApiCompatibilityLevel => "X-EBAY-API-COMPATIBILITY-LEVEL",
            EbayXmlHeader::XEbayApiDevName => "X-EBAY-API-DEV-NAME",
            EbayXmlHeader::XEbayApiAppName => "X-EBAY-API-APP-NAME",
            EbayXmlHeader::XEbayApiCertName => "X-EBAY-API-CERT-NAME",
            EbayXmlHeader::XEbayApiCallName => "X-EBAY-API-CALL-NAME",
            EbayXmlHeader::XEbayApiSiteId => "X-EBAY-API-SITEID",
        }
    }
}

#[allow(unused)]
pub enum WarningLevel {
    High,
    Low,
}

impl WarningLevel {
    pub fn get_serial_value(&self) -> String {
        match self {
            WarningLevel::High => "High".to_string(),
            WarningLevel::Low => "Low".to_string(),
        }
    }
}
