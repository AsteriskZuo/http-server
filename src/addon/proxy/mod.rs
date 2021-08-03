pub mod handler;
pub mod target;

use http::header::{HeaderName, HeaderValue};
use http::{Request, Response};
use hyper::client::HttpConnector;
use hyper::{Body, Client, Method, Uri};
use hyper_tls::HttpsConnector;
use std::sync::Arc;
use tokio::sync::Mutex;

use self::target::Target;

pub type HttpClient = Client<HttpsConnector<HttpConnector>>;

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
pub const DYNAMIC_PROXY_URL_HEADER: &str = "X-Proxy-URL";

/// Dynamic proxy header to extract HTTP Request method from
pub const DYNAMIC_PROXY_METHOD_HEADER: &str = "X-Proxy-Method";

/// Dynamic proxy header to extract HTTP request "Autorization" header
/// to use against proxied server
pub const DYNAMIC_PROXY_AUTHORIZATION_HEADER: &str = "X-Proxy-Authorization";

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
    http_client: Arc<HttpClient>,
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
        self.remove_hop_by_hop_headers(Arc::clone(&request)).await;

        self.proxy(Arc::clone(&request)).await
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

    async fn proxy(&self, request: Arc<Mutex<Request<Body>>>) -> Response<Body> {
        let request = request.lock().await;
        let http_client = Arc::clone(&self.http_client);

        match &self.kind {
            Kind::Static(target) => target.perform(http_client).await,
            Kind::Dynamic => {
                let headers = request.headers();
                let target = Target::from_dynamic_proxy_headers(headers).await.unwrap();

                target.perform(http_client).await
            }
        }
    }
}
