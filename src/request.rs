use bytes::Buf;
use http_body_util::BodyExt;
use hyper::header::CONTENT_TYPE;
use hyper::{StatusCode, Uri};
use hyper_util::client::legacy::{connect::Connect, Client};
use std::io::Read;
use std::str::FromStr;

use crate::Error;

pub async fn send_request<C, T>(
    client: &Client<C, String>,
    method: &str,
    request_url: url::Url,
    data: Option<String>,
) -> Result<T, Error>
where
    C: Connect + Clone + Send + Sync + 'static,
    T: serde::de::DeserializeOwned,
{
    let request_uri: Uri = FromStr::from_str(request_url.as_str()).unwrap();
    let request_builder = hyper::Request::builder()
        .method(method)
        .uri(request_uri)
        .header(CONTENT_TYPE, "application/json");
    let request = match data {
        Some(body) => request_builder.body(body)?,
        None => request_builder.body(String::new())?,
    };

    let response = client.request(request).await?;
    let status = response.status();
    let mut body_reader = response.collect().await?.aggregate().reader();

    match status {
        StatusCode::OK => {
            let body = serde_json::from_reader(body_reader).unwrap();
            Ok(body)
        }
        _ => {
            let mut body = String::new();
            body_reader.read_to_string(&mut body).unwrap();
            Err(Error::Response(status, body))
        }
    }
}
