use anyhow::Result;
use http::response::Builder as HttpResponseBuilder;
use http::StatusCode;
use hyper::{Body, Method, Request, Response};
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::Mutex;

mod service;
mod redis;

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
          return self.health();
        } else if req_path.contains("/api/v1/navi") {
          let list: Vec<_> = req_path.as_str().split(':').collect();
          let mut id = String::new();
          if 2 == list.len() {
            id = list[1].to_string();
          }
          return self.get_id(id);
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
  fn health(&self) -> Result<Response<Body>, StatusCode> {
    Ok(
      HttpResponseBuilder::new()
        .header(http::header::CONTENT_TYPE, "text/html")
        .status(StatusCode::OK)
        .body(Body::from("health"))
        .expect("Failed to build response"),
    )
  }
  fn get_id(&self, id: String) -> Result<Response<Body>, StatusCode> {
    match service::get_result(&id) {
      Ok(ret) => Ok(
        HttpResponseBuilder::new()
          .header(http::header::CONTENT_TYPE, "application/octet-stream")
          .status(StatusCode::OK)
          .body(Body::from(ret.to_vec()))
          .expect("Failed to build response"),
      ),
      Err(error) => {
        println!("get_id>ret={}", error);
        Err(StatusCode::INTERNAL_SERVER_ERROR)
      }
    }
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
