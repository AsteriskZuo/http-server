use anyhow::{Error, Result};
use http::header::HeaderValue;
use http::request::Builder as HttpRequestBuilder;
use http::response::Builder as HttpResponseBuilder;
use http::{HeaderMap, Request, Response};
use hyper::client::HttpConnector;
use hyper::server::conn::Http;
use hyper::{Body, Client, Method, Uri};
use hyper_tls::HttpsConnector;
use std::convert::TryFrom;
use std::sync::Arc;
use tokio::sync::oneshot;

use super::{
    HttpClient, DYNAMIC_PROXY_AUTHORIZATION_HEADER, DYNAMIC_PROXY_METHOD_HEADER,
    DYNAMIC_PROXY_URL_HEADER,
};

#[derive(Clone, Debug)]
pub struct Target {
    pub(crate) url: Uri,
    pub(crate) method: Method,
    pub(crate) authorization: Option<HeaderValue>,
}

impl Target {
    pub async fn perform(&self, http_client: Arc<HttpClient>) -> Response<Body> {
        let request = Request::try_from(self).unwrap();
        let (tx, rx) = oneshot::channel();

        tokio::spawn(async move {
            let response = http_client.request(request).await;

            tx.send(response).unwrap();
        });

        if let Ok(response) = rx.await.unwrap() {
            return response;
        }

        HttpResponseBuilder::new().body(Body::empty()).unwrap()
    }

    pub async fn from_dynamic_proxy_headers(headers: &HeaderMap) -> Result<Self> {
        let url: Uri = headers
            .get(DYNAMIC_PROXY_URL_HEADER)
            .ok_or(Error::msg("Missing \"X-Proxy-URL\" header"))?
            .to_str()
            .map_err(|err| Error::msg(err.to_string()))?
            .parse()
            .map_err(|err: http::uri::InvalidUri| Error::msg(err.to_string()))?;

        let method: Method = headers
            .get(DYNAMIC_PROXY_METHOD_HEADER)
            .ok_or(Error::msg("Missing \"X-Proxy-Method\" header"))?
            .to_str()
            .map_err(|err| Error::msg(err.to_string()))?
            .parse()
            .map_err(|err: http::method::InvalidMethod| Error::msg(err.to_string()))?;

        let authorization = match headers.get(DYNAMIC_PROXY_AUTHORIZATION_HEADER) {
            Some(header) => Some(header.to_owned()),
            None => None,
        };

        Ok(Target {
            url,
            method,
            authorization,
        })
    }
}

impl TryFrom<&Target> for Request<Body> {
    type Error = Error;

    fn try_from(target: &Target) -> Result<Self, Self::Error> {
        HttpRequestBuilder::new()
            .uri(target.url.clone())
            .method(target.method.clone())
            .body(Body::empty())
            .map_err(|err| Error::msg(err.to_string()))
    }
}
