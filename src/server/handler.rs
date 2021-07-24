use anyhow::Result;
use http::{Request, Response};
use hyper::Body;
use std::convert::TryFrom;
use std::sync::Arc;

use crate::addon::file_server::handler::{make_file_server_handler, FileServerHandler};
use crate::addon::proxy::handler::make_proxy_handler;
use crate::addon::proxy::Proxy;
use crate::Config;

use super::middleware::{Handler, Middleware};

#[derive(Clone)]
pub struct HttpHandler {
    handler: Arc<Handler>,
    middleware: Arc<Middleware>,
}

impl HttpHandler {
    pub async fn handle_request(self, request: Request<Body>) -> Result<Response<Body>> {
        let middleware = Arc::clone(&self.middleware);
        let response = middleware.handle(request, Arc::clone(&self.handler)).await;

        Ok(response)
    }

    pub fn handler_from_config(config: Arc<Config>) -> Handler {
        if let Some(proxy_config) = config.proxy() {
            let proxy =
                Proxy::try_from(proxy_config).expect("Invalid configuration provided to Proxy");
            let proxy = Arc::new(proxy);

            return make_proxy_handler(proxy);
        }

        let file_server = FileServerHandler::new(config.root_dir());
        let file_server = Arc::new(file_server);

        return make_file_server_handler(file_server);
    }
}

impl From<Arc<Config>> for HttpHandler {
    fn from(config: Arc<Config>) -> Self {
        let handler = HttpHandler::handler_from_config(Arc::clone(&config));
        let handler = Arc::new(handler);
        let middleware = Middleware::try_from(config).unwrap();
        let middleware = Arc::new(middleware);

        HttpHandler {
            handler,
            middleware,
        }
    }
}
