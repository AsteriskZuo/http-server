use anyhow::Result;
use http::response::Builder as HttpResponseBuilder;
use http::StatusCode;
use hyper::{Body, Method, Request, Response};
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::Mutex;

mod service;

#[derive(Default, Debug)]
pub struct ApiServer {
  root_dir: PathBuf,
}

impl<'a> ApiServer {
  /// Creates a new instance of the `FileExplorer` with the provided `root_dir`
  pub fn new(root_dir: PathBuf) -> Self {
    ApiServer { root_dir }
  }

  /// Resolves a HTTP Request to a api.
  pub async fn resolve(
    &self,
    request: Arc<Mutex<Request<Body>>>,
  ) -> Result<Response<Body>, StatusCode> {
    let request_lock = request.lock().await;
    let req_path = request_lock.uri().to_string();
    let req_method = request_lock.method();
    match *req_method {
      Method::GET => {
        if req_path.contains("/api/v1/health") {
          return self.health(Arc::clone(&request));
        } else if req_path.contains("/api/v1/navi") {
          return self.get_id(Arc::clone(&request));
        } else {
          return Err(StatusCode::CONTINUE);
        }
      }
      Method::POST => {
        if req_path.contains("/api/v1/navi") {
          return self.get_path(Arc::clone(&request));
        } else {
          return Err(StatusCode::CONTINUE);
        }
      }
      _ => return Err(StatusCode::CONTINUE),
    }
  }
}

impl ApiServer {
  fn health(&self, request: Arc<Mutex<Request<Body>>>) -> Result<Response<Body>, StatusCode> {
    Ok(
      HttpResponseBuilder::new()
        .header(http::header::CONTENT_TYPE, "text/html")
        .status(StatusCode::OK)
        .body(Body::from("health"))
        .expect("Failed to build response"),
    )
  }
  fn get_id(&self, request: Arc<Mutex<Request<Body>>>) -> Result<Response<Body>, StatusCode> {
    Ok(
      HttpResponseBuilder::new()
        .header(http::header::CONTENT_TYPE, "text/html")
        .status(StatusCode::OK)
        .body(Body::from("get_id"))
        .expect("Failed to build response"),
    )
  }
  fn get_path(&self, request: Arc<Mutex<Request<Body>>>) -> Result<Response<Body>, StatusCode> {
    Ok(
      HttpResponseBuilder::new()
        .header(http::header::CONTENT_TYPE, "text/html")
        .status(StatusCode::OK)
        .body(Body::from("get_path"))
        .expect("Failed to build response"),
    )
  }
}
