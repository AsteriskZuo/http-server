use anyhow::Result;
use http::response::Builder as HttpResponseBuilder;
use http::StatusCode;
use hyper::{Body, Method, Request, Response};
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::config::Config;

mod redis_client;
mod service;

#[derive(Default, Debug)]
pub struct ApiServer {
  // root_dir: PathBuf,
  services: Arc<service::Service>,
}

impl<'a> ApiServer {
  /// Creates a new instance of the `FileExplorer` with the provided `root_dir`
  pub fn new(config: Arc<Config>) -> Self {
    ApiServer {
      services: Arc::new(service::Service::new(Arc::clone(&config))),
    }
  }

  /// Resolves a HTTP Request to a api.
  pub async fn resolve(
    &self,
    request: Arc<Mutex<Request<Body>>>,
  ) -> Result<Response<Body>, StatusCode> {
    let request_lock = request.lock().await;
    let mut req_path = request_lock.uri().to_string();
    let req_method = request_lock.method();
    match *req_method {
      Method::GET => {
        if req_path.contains("/api/v1/health") {
          return self.health();
        } else if req_path.contains("/api/v1/navi") {
          let pos = req_path.find(':');
          match pos {
            Some(pos) => {
              let id = req_path.split_off(pos + 1);
              return self.get_id(id);
            }
            None => {
              return Err(StatusCode::CONTINUE);
            }
          }
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
    let mut services_arc = self.services.clone();
    let services_mut = Arc::make_mut(&mut services_arc);
    match services_mut.get_value(&id) {
      Some(ret) => Ok(
        HttpResponseBuilder::new()
          .header(http::header::CONTENT_TYPE, "application/octet-stream")
          .status(StatusCode::OK)
          .body(Body::from(ret))
          .expect("Failed to build response"),
      ),
      None => {
        println!("get_id>ret=");
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
