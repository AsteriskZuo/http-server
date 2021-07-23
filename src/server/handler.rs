use anyhow::Result;
use http::{Request, Response};
use hyper::Body;
use std::convert::TryFrom;
use std::sync::Arc;

use crate::addon::file_server::handler::{make_file_server_handler, FileServerHandler};
use crate::addon::proxy::handler::make_proxy_handler;
use crate::addon::proxy::{Kind, Proxy, Target};
use crate::Config;

use super::middleware::Middleware;

#[derive(Clone)]
pub struct HttpHandler {
    file_explorer: Arc<FileServerHandler>,
    middleware: Arc<Middleware>,
}

impl HttpHandler {
    pub async fn handle_request(self, request: Request<Body>) -> Result<Response<Body>> {
        // let handler = make_file_server_handler(self.file_explorer);
        let proxy = Arc::new(Proxy::from());
        let handler = make_proxy_handler(proxy);
        let middleware = Arc::clone(&self.middleware);
        let response = middleware.handle(request, handler).await;

        Ok(response)
    }
}

impl From<Arc<Config>> for HttpHandler {
    fn from(config: Arc<Config>) -> Self {
        let file_explorer = Arc::new(FileServerHandler::new(config.root_dir()));
        let middleware = Middleware::try_from(config).unwrap();
        let middleware = Arc::new(middleware);

        HttpHandler {
            file_explorer,
            middleware,
        }
    }
}
