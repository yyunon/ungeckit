use bytes::Bytes;
use reqwest::{self, header, Method, Request, RequestBuilder, Url};

use crate::utils::error::{ErrorKind, GeckError};

pub async fn request<'a>(
    client: &reqwest::Client,
    verb: &str,
    url: &str,
    data: String,
) -> Result<Bytes, GeckError> {
    let method = Method::from_bytes(verb.as_bytes()).unwrap();
    let mut headers = header::HeaderMap::new();
    headers.insert("Accept", "application/json".parse().unwrap());
    headers.insert(
        "Content-Type",
        "application/json;charset=UTF-8".parse().unwrap(),
    );
    let request = Request::new(method, Url::parse(&url).expect("Cannot parse url"));
    let response = RequestBuilder::from_parts(client.clone(), request)
        .headers(headers)
        .body(data)
        .send()
        .await?;
    if response.status() == 200 {
        Ok(response.bytes().await?)
    } else {
        Err(GeckError::new(
            ErrorKind::Service,
            None::<GeckError>,
            &format!("Response status is {}", response.status()),
        ))
    }
}
