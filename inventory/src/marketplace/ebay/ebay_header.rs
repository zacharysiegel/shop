use actix_web::error::ParseError;
use actix_web::http::header::{HeaderName, HeaderValue, InvalidHeaderValue, TryIntoHeaderValue};
use actix_web::HttpMessage;

/// To eventually authenticate `inventory` requests with eBay, the client must provide an x-ebay-authorization
///     header along with the 
pub struct XEbayAuthorization {
    value: String,
}

impl TryIntoHeaderValue for XEbayAuthorization {
    type Error = InvalidHeaderValue;

    fn try_into_value(self) -> Result<HeaderValue, Self::Error> {
        HeaderValue::from_str(&self.value)
    }
}

impl actix_web::http::header::Header for XEbayAuthorization {
    fn name() -> HeaderName {
        HeaderName::from_static("x-ebay-authorization")
    }

    fn parse<M: HttpMessage>(msg: &M) -> Result<Self, ParseError> {
        let header_value: &HeaderValue = match msg.headers().get(Self::name()) {
            Some(value) => value,
            None => return Err(ParseError::Header),
        };
        let header = XEbayAuthorization {
            value: header_value.to_str()
                .map_err(|_| ParseError::Header)?
                .to_string(),
        };
        Ok(header)
    }
}
