
use http::response::Builder as HttpResponseBuilder;
use http::StatusCode;
use hyper::{Body, Method, Request};
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::addon::api_server::ApiServer;
use crate::addon::file_server::FileServer;

use super::Handler;
use super::ServerHandler;

#[derive(Default)]
pub struct ApiServerHandler {
  api_server: Arc<ApiServer>,
}

impl ServerHandler for ApiServerHandler {
  fn new_file(file_server: FileServer) -> Self {
    panic!("can't create ServerHandler {:?}", file_server)
  }
  fn new_api(api_server: ApiServer) -> Self {
    let api_server = Arc::new(Default::default());
    ApiServerHandler { api_server }
  }

  fn handle(&self) -> Handler {
    let api_server = Arc::clone(&self.api_server);

    Box::new(move |request: Arc<Mutex<Request<Body>>>| {
      let api_server = Arc::clone(&api_server);
      let request = Arc::clone(&request);

      Box::pin(async move {
        let api_server = Arc::clone(&api_server);
        let request = Arc::clone(&request);
        let request_lock = request.lock().await;
        let req_path = request_lock.uri().to_string();
        let req_method = request_lock.method();

        if req_method == Method::GET {
          return api_server
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
