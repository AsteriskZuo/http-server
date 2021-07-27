pub mod handler;

use anyhow::{Error, Result};
use http::header::{HeaderName, HeaderValue};
use http::request::Builder as HttpRequestBuilder;
use http::response::Builder as HttpResponseBuilder;
use http::{HeaderMap, Request, Response, StatusCode};
use hyper::client::HttpConnector;
use hyper::{Body, Client, Method, Uri};
use hyper_tls::HttpsConnector;
use std::convert::TryFrom;
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::utils::error::make_http_error_response;

/// End-to-end and Hop-by-hop Headers
///
/// For the purpose of defining the behavior of caches and non-caching proxies,
/// we divide HTTP headers into two categories:
///
///       - End-to-end headers, which are  transmitted to the ultimate
///         recipient of a request or response. End-to-end headers in
///         responses MUST be stored as part of a cache entry and MUST be
///         transmitted in any response formed from a cache entry.
///       - Hop-by-hop headers, which are meaningful only for a single
///         transport-level connection, and are not stored by caches or
///         forwarded by proxies.
///
/// The following HTTP/1.1 headers are hop-by-hop headers:
///
///       - Connection
///       - Keep-Alive
///       - Proxy-Authenticate
///       - Proxy-Authorization
///       - TE
///       - Trailers
///       - Transfer-Encoding
///       - Upgrade
/// All other headers defined by HTTP/1.1 are end-to-end headers.
///
/// Refer: https://www.w3.org/Protocols/rfc2616/rfc2616-sec13.html (13.5.1)
const HOP_BY_HOP_HEADERS: [&str; 8] = [
    "Connection",
    "Keep-Alive",
    "Proxy-Authentication",
    "Proxy-Authorization",
    "Te",
    "Trailers",
    "Transfer-Encoding",
    "Upgrade",
];

/// Dynamic proxy header to extract HTTP request URL from
const DYNAMIC_PROXY_URL_HEADER: &str = "X-Proxy-URL";

/// Dynamic proxy header to extract HTTP Request method from
const DYNAMIC_PROXY_METHOD_HEADER: &str = "X-Proxy-Method";

/// Dynamic proxy header to extract HTTP request "Autorization" header
/// to use against proxied server
const DYNAMIC_PROXY_AUTHORIZATION_HEADER: &str = "X-Proxy-Authorization";

#[derive(Clone, Debug)]
pub struct Target {
    url: Uri,
    method: Method,
    authorization: Option<HeaderValue>,
}

impl Target {
    pub async fn perform(
        &self,
        client: Arc<Client<HttpsConnector<HttpConnector>>>,
    ) -> Result<Response<Body>> {
        let request = Request::try_from(self)?;
        let response = client.request(request).await;

        return Ok(HttpResponseBuilder::new().body(Body::empty()).unwrap());
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
            .uri(target.url)
            .method(target.method)
            .body(Body::empty())
            .map_err(|err| Error::msg(err.to_string()))
    }
}

/// Represents the target kind for an instance of `Proxy`.
/// Two main kinds are supported, `Dynamic` and `Static`.
///
/// - `Static` Target: The proxy target is set once during proxy configuration,
/// and is used for the complete lifecycle of the proxy
///
/// - `Dynamic` Target: The proxy target is taken from HTTP Headers (these
/// are defined in the `DYNAMIC_PROXY_HEADERS` array).
/// Every request should specify the target URL, target Method and any other
/// relevant headers
#[derive(Clone, Debug)]
pub enum Kind {
    Static(Target),
    Dynamic,
}

pub struct Proxy {
    http_client: Arc<Client<HttpsConnector<HttpConnector>>>,
    kind: Kind,
}

impl Proxy {
    pub fn new_dynamic() -> Self {
        Proxy {
            http_client: Arc::new(Client::builder().build(HttpsConnector::new())),
            kind: Kind::Dynamic,
        }
    }

    pub fn new_static(url: Uri, method: Method, authorization: Option<HeaderValue>) -> Self {
        Proxy {
            http_client: Arc::new(Client::builder().build(HttpsConnector::new())),
            kind: Kind::Static(Target {
                url,
                method,
                authorization,
            }),
        }
    }

    /// Handles a HTTP request through the Proxy
    pub async fn handle(&self, request: Arc<Mutex<Request<Body>>>) -> Response<Body> {
        self.append_x_forwarded_for(Arc::clone(&request)).await;
        self.remove_hop_by_hop_headers(Arc::clone(&request)).await;

        if let Err(err) = self.proxy(Arc::clone(&request)).await {
            return make_http_error_response(StatusCode::BAD_REQUEST, err.to_string().as_str());
        }

        HttpResponseBuilder::new()
            .status(200)
            .body(Body::empty())
            .unwrap()
    }

    /// Creates a new HTTP Request with the same defintion as
    /// the original one by with Hop-by-Hop headers removed
    async fn remove_hop_by_hop_headers(&self, request: Arc<Mutex<Request<Body>>>) {
        let mut request = request.lock().await;
        let headers = request.headers_mut();

        for header in HOP_BY_HOP_HEADERS.iter() {
            let header: HeaderName = header.parse().unwrap();

            if headers.contains_key::<HeaderName>(header.clone()) {
                headers.remove(header);
            }
        }
    }

    async fn append_x_forwarded_for(&self, request: Arc<Mutex<Request<Body>>>) {
        let mut request = request.lock().await;
        let headers = request.headers_mut();

        // TODO: Extract host IP from Request and assign it to X-Forwarded-For
        headers.append(
            HeaderName::from_static("X-Forwarded-For"),
            HeaderValue::from_bytes(b"FAKE FIXME").unwrap(),
        );
    }

    async fn proxy(&self, request: Arc<Mutex<Request<Body>>>) -> Result<Response<Body>> {
        let request = request.lock().await;

        match &self.kind {
            Kind::Static(target) => Ok(target.perform(Arc::clone(&self.http_client)).await?),
            Kind::Dynamic => {
                let headers = request.headers();
                let target = Target::from_dynamic_proxy_headers(headers).await?;

                Ok(target.perform(Arc::clone(&self.http_client)).await?)
            }
        }
    }
}
