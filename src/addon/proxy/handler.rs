use hyper::{Body, Request};
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::server::middleware::Handler;

use super::Proxy;

/// Creates a `middleware::Handler` which makes use of the provided `FileExplorer`
pub fn make_proxy_handler(proxy: Arc<Proxy>) -> Handler {
    Box::new(move |request: Arc<Mutex<Request<Body>>>| {
        let proxy = Arc::clone(&proxy);

        Box::pin(async move { proxy.handle(request).await })
    })
}
