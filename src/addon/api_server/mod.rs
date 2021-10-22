use anyhow::Result;
use http::response::Builder as HttpResponseBuilder;
use http::StatusCode;
use hyper::{body::HttpBody, Body, Method, Request, Response};
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::config::Config;

mod proto_wrapper;
mod redis_client;
mod route_wrapper;
mod search_poi;
mod service;

#[derive(Default, Debug)]
pub struct ApiServer {
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
    let mut request_lock = request.lock().await;
    let mut req_path = request_lock.uri().to_string();
    let req_method = request_lock.method();
    println!("{:?}{:?}", req_path.to_string(), req_method.to_string());
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
          let orb = request_lock.body_mut().data().await;
          match orb {
            Some(result) => match result {
              Ok(bytes) => {
                let body_data = String::from_utf8(bytes.to_vec()).expect("body_data");
                println!("{:?}", body_data);
                return self.get_path(body_data).await;
              }
              Err(error) => {
                println!("search_poi_info->{:?}", error);
                return Err(StatusCode::CONTINUE);
              }
            },
            None => {
              println!("search_poi_info->None");
              return Err(StatusCode::CONTINUE);
            }
          }

          // let pos = req_path.find('?');
          // match pos {
          //   Some(pos) => {
          //     let data = req_path.split_off(pos + 1);
          //     return self.get_path(data).await;
          //   }
          //   None => {
          //     return Err(StatusCode::CONTINUE);
          //   }
          // }
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
  async fn get_path(&self, raw_data: String) -> Result<Response<Body>, StatusCode> {
    let result = self.services.find_path(raw_data).await;
    match result {
      Ok(ret) => {
        let mut services_arc = self.services.clone();
        let services_mut = Arc::make_mut(&mut services_arc);
        // let redis_result = services_mut.save_value(&ret.0, &ret.1);
        // match redis_result {
        //   Ok(_) => {},
        //   Err(error) => {
        //     println!("get_path>ret={}", error);
        //   }
        // }
        Ok(
          HttpResponseBuilder::new()
            .header(http::header::CONTENT_TYPE, "text/html")
            .status(StatusCode::OK)
            .body(Body::from(ret.1))
            .expect("Failed to build response"),
        )
      }
      Err(error) => {
        println!("get_id>ret={}", error);
        Err(StatusCode::INTERNAL_SERVER_ERROR)
      }
    }
  }
}

#[cfg(test)]
pub mod tests {
    use crate::protos::route_client_param::RoutePlanClientParameter;

  #[test]
  fn test_get_path() {
    use super::*;
    let config = Arc::new(Config::default());
    let server = ApiServer::new(config);
    // let raw_data;
    let client_params = RoutePlanClientParameter::new();
    // client_p
    // server.get_path(raw_data);
  }
}