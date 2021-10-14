use http::response::Builder as HttpResponseBuilder;
use http::StatusCode;
use hyper::{Body, Request};
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
    ApiServerHandler {
      api_server: Arc::new(api_server),
    }
  }

  fn handle(&self) -> Handler {
    let api_server = Arc::clone(&self.api_server);

    Box::new(move |request: Arc<Mutex<Request<Body>>>| {
      let api_server = Arc::clone(&api_server);
      
      Box::pin(async move {
        match api_server.resolve(request).await {
          Ok(ret) => ret,
          Err(e) => {
            let ret = HttpResponseBuilder::new()
              .status(StatusCode::NOT_FOUND)
              .header(http::header::CONTENT_TYPE, "text/html")
              .body(Body::from(e.to_string()))
              .expect("Unable to build response");
            println!("{:?}", ret);
            return ret;
          }
        }
        // api_server
        //   .resolve(request)
        //   .await
        //   .map_err(|e| {
        //     HttpResponseBuilder::new()
        //       .status(e)
        //       .body(Body::from(e.to_string()))
        //       .expect("Unable to build response")
        //   })
        //   .unwrap()
      })
    })
  }
}
