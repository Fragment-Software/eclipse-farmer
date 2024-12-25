use reqwest::{Method, Proxy};

use crate::utils::fetch::{send_http_request, RequestParams};

use super::{
    constants::COLLECTIONS_URL,
    schema::{CollectionsBody, CollectionsResponse},
};

pub async fn get_create_nft_tx(
    body: CollectionsBody,
    proxy: Option<&Proxy>,
) -> eyre::Result<CollectionsResponse> {
    let request_params = RequestParams {
        url: COLLECTIONS_URL,
        method: Method::POST,
        body: Some(body),
        query_args: None,
        proxy,
        headers: None,
    };

    let response_body = send_http_request::<CollectionsResponse>(request_params).await?;

    Ok(response_body)
}
