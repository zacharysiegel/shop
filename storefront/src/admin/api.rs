use crate::admin::structure::error_text::error_markup;
use crate::registry::REGISTRY;
use maud::Markup;
use reqwest::RequestBuilder;
use serde::de::DeserializeOwned;

pub async fn wrapped_get<SerialT: DeserializeOwned>(path_and_query: &str) -> Result<SerialT, Markup> {
    let result = REGISTRY.http_client
        .get(format!("{}{}", REGISTRY.remote_url, path_and_query))
        .send()
        .await;
    let response = match result {
        Ok(response) => response,
        Err(error) => {
            return Err(error_markup(error));
        }
    };
    let serial = match response.json::<SerialT>().await {
        Ok(element) => element,
        Err(error) => {
            return Err(error_markup(error));
        }
    };
    Ok(serial)
}

pub async fn wrapped_get_cookie<SerialT: DeserializeOwned>(
    path_and_query: &str,
    cookie: Option<&actix_web::http::header::HeaderValue>
) -> Result<SerialT, Markup> {
    log::info!("{}", cookie.unwrap().to_str().unwrap());
    let request_builder: RequestBuilder = REGISTRY.http_client
        .get(format!("{}{}", REGISTRY.remote_url, path_and_query));

    let request_builder = match cookie {
        Some(cookie) => request_builder.header("Cookie", cookie.to_str().unwrap()),
        None => request_builder,
    };

    let result = request_builder
        .send()
        .await;
    let response = match result {
        Ok(response) => response,
        Err(error) => {
            return Err(error_markup(error));
        }
    };
    let serial = match response.json::<SerialT>().await {
        Ok(element) => element,
        Err(error) => {
            return Err(error_markup(error));
        }
    };
    Ok(serial)
}
