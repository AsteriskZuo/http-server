mod api_server;
mod file_server;

use anyhow::Result;
use futures::Future;
use http::{Request, Response};
use hyper::Body;
use std::convert::TryFrom;
use std::pin::Pin;
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::addon::api_server::ApiServer;
use crate::addon::file_server::FileServer;
use crate::config::ServerType;
use crate::Config;

use super::middleware::Middleware;

use self::api_server::ApiServerHandler;
use self::file_server::FileServerHandler;

/// The main handler for the HTTP request, a HTTP response is created
/// as a result of this handler.
///
/// This handler will be executed against the HTTP request after every
/// "Middleware Before" chain is executed but before any "Middleware After"
/// chain is executed
pub type Handler = Box<
    dyn Fn(
            Arc<Mutex<Request<Body>>>,
        ) -> Pin<Box<dyn Future<Output = http::Response<Body>> + Send + Sync>>
        + Send
        + Sync,
>;

pub trait ServerHandler {
    fn new_file(file_server: FileServer) -> Self;
    fn new_api(api_server: ApiServer) -> Self;
    fn handle(&self) -> Handler;
}

#[derive(Clone)]
pub struct HttpHandler {
    file_server_handler: Arc<FileServerHandler>,
    api_server_handler: Arc<ApiServerHandler>,
    middleware: Arc<Middleware>,
    server_type: ServerType,
}

impl HttpHandler {
    fn new(config: Arc<Config>) -> Self {
        let middleware = Middleware::try_from(Arc::clone(&config)).unwrap();
        let middleware = Arc::new(middleware);
        match config.action() {
            ServerType::ApiServices => {
                let api_server = ApiServer::new(Arc::clone(&config));
                HttpHandler {
                    middleware: middleware,
                    file_server_handler: Default::default(),
                    api_server_handler: Arc::new(ApiServerHandler::new_api(api_server)),
                    server_type: config.action(),
                }
            }
            ServerType::FileServices => {
                let file_server = FileServer::new(config.root_dir());
                let file_server_handler = Arc::new(FileServerHandler::new_file(file_server));
                HttpHandler {
                    middleware: middleware,
                    file_server_handler: file_server_handler,
                    api_server_handler: Default::default(),
                    server_type: config.action(),
                }
            }
            ServerType::UnknownServices => panic!("not support this type"),
        }
    }
    pub async fn handle_request(self, request: Request<Body>) -> Result<Response<Body>> {
        match self.server_type {
            ServerType::ApiServices => {
                let handler = Arc::clone(&self.api_server_handler);
                let middleware = Arc::clone(&self.middleware);
                let response = middleware.handle(request, handler.handle()).await;
                Ok(response)
            }
            ServerType::FileServices => {
                let handler = Arc::clone(&self.file_server_handler);
                let middleware = Arc::clone(&self.middleware);
                let response = middleware.handle(request, handler.handle()).await;
                Ok(response)
            }
            ServerType::UnknownServices => panic!("not support this type"),
        }
    }
}

impl From<Arc<Config>> for HttpHandler {
    fn from(config: Arc<Config>) -> Self {
        // HttpHandler::new(Arc::clone(&config))
        HttpHandler::new(config)
    }
}
