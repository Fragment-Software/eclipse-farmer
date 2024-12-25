use std::collections::HashMap;

use reqwest::{header::HeaderMap, Client, Method};
use serde::{de::DeserializeOwned, Serialize};

#[derive(Clone)]
pub struct RequestParams<'a, S: Serialize> {
    pub url: &'a str,
    pub method: Method,
    pub body: Option<S>,
    pub query_args: Option<HashMap<&'a str, &'a str>>,
    pub proxy: Option<&'a reqwest::Proxy>,
    pub headers: Option<HeaderMap>,
}

pub async fn send_http_request<R: DeserializeOwned>(
    request_params: RequestParams<'_, impl Serialize>,
) -> eyre::Result<R> {
    let client = request_params.proxy.map_or_else(Client::new, |proxy| {
        Client::builder().proxy(proxy.clone()).build().unwrap_or_else(|err| {
            tracing::error!("Failed to build a client with proxy: {proxy:?}. Error: {err}");
            Client::new()
        })
    });

    let mut request = client.request(request_params.method.clone(), request_params.url);

    if let Some(params) = &request_params.query_args {
        request = request.query(&params);
    }

    if let Some(body) = &request_params.body {
        request = request.json(&body);
    }

    if let Some(headers) = request_params.headers.as_ref() {
        request = request.headers(headers.clone());
    }

    let response =
        request.send().await.inspect_err(|e| tracing::error!("Request failed: {}", e))?;

    let text = response
        .text()
        .await
        .inspect_err(|e| tracing::error!("Failed to retrieve response text: {}", e))?;

    let deserialized_body = serde_json::from_str::<R>(&text)
        .inspect_err(|e| tracing::error!("Failed to deserialize response: {}\n {} ", e, text))?;

    Ok(deserialized_body)
}
