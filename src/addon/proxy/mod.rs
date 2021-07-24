pub mod handler;

use http::header::{HeaderName, HeaderValue};
use http::response::Builder as HttpResponseBuilder;
use http::{Request, Response};
use hyper::{Body, Client, Method, Uri};
use serde::Deserialize;
use std::str::FromStr;
use std::sync::Arc;
use tokio::sync::Mutex;

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

const DYNAMIC_PROXY_HEADERS: [&str; 3] = ["X-Proxy-URL", "X-Proxy-Method", "X-Proxy-Authorization"];

#[derive(Clone, Debug)]
pub struct Target {
    url: Uri,
    method: Method,
    authorization: Option<HeaderValue>,
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
    kind: Kind,
}

impl Proxy {
    pub fn new_dynamic() -> Self {
        Proxy {
            kind: Kind::Dynamic,
        }
    }

    pub fn new_static(url: Uri, method: Method, authorization: Option<HeaderValue>) -> Self {
        Proxy {
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
        self.proxy(Arc::clone(&request)).await;

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
            HeaderValue::from_bytes(b"192.168.0.1").unwrap(),
        );
    }

    async fn proxy(&self, request: Arc<Mutex<Request<Body>>>) {
        let mut request = request.lock().await;

        match &self.kind {
            Kind::Static(target) => {
                self.perform_request(target.url.to_string().as_str()).await;
            }
            Kind::Dynamic => {
                let headers = request.headers_mut();

                if let Some(url) = headers.get(HeaderName::from_str("X-Proxy-URL").unwrap()) {
                    self.perform_request(url.to_str().unwrap()).await;
                }
            }
        }
    }

    async fn perform_request(&self, url: &str) -> Response<Body> {
        todo!()
    }
}
