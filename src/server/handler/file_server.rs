use http::response::Builder as HttpResponseBuilder;
use http::StatusCode;
use hyper::{Body, Method, Request};
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::addon::file_server::FileServer;
use crate::addon::api_server::ApiServer;

use super::Handler;
use super::ServerHandler;

#[derive(Default)]
pub struct FileServerHandler {
    file_server: Arc<FileServer>,
}

impl ServerHandler for FileServerHandler {
    fn new_file(file_server: FileServer) -> Self {
        let file_server = Arc::new(file_server);

        FileServerHandler { file_server }
    }
    fn new_api(api_server: ApiServer) -> Self {
        panic!("can't create ServerHandler {:?}", api_server)
    }

    fn handle(&self) -> Handler {
        let file_server = Arc::clone(&self.file_server);

        Box::new(move |request: Arc<Mutex<Request<Body>>>| {
            let file_server = Arc::clone(&file_server);
            let request = Arc::clone(&request);

            println!("req:{:?}", request);

            Box::pin(async move {
                let file_server = Arc::clone(&file_server);
                let request = Arc::clone(&request);
                let request_lock = request.lock().await;
                let req_path = request_lock.uri().to_string();
                let req_method = request_lock.method();

                if req_method == Method::GET {
                    return file_server
                        .resolve(req_path)
                        .await
                        .map_err(|e| {
                            HttpResponseBuilder::new()
                                .status(StatusCode::INTERNAL_SERVER_ERROR)
                                .body(Body::from(e.to_string()))
                                .expect("Unable to build response")
                        })
                        .unwrap();
                }

                HttpResponseBuilder::new()
                    .status(StatusCode::METHOD_NOT_ALLOWED)
                    .body(Body::empty())
                    .expect("Unable to build response")
            })
        })
    }
}
