use std::sync::Arc;

use criterion::Criterion;
use criterion::{criterion_group, criterion_main};
use http::{Method, Request};
use hyper::client::HttpConnector;
use hyper::{Body, Client};
use lazy_static::lazy_static;
use tokio::runtime::Runtime;

lazy_static! {
  static ref HTTP_CLIENT: Client<HttpConnector> = Client::new();
}

async fn request_post() {
  // let client = Client::new();
  let file_contents = r#"
  {
      "version": 1,
      "startPoint": {
          "longitude": 116.4418912826,
          "latitude": 39.9090135175,
          "height": 0
      },
      "endPoint": {
          "longitude": 116.4370057383,
          "latitude": 39.9152984703,
          "height": 0
      }
  }
"#;

  let req = Request::builder()
    .method(Method::POST)
    .uri("http://127.0.0.1:7878/api/v1/navijson")
    .header("Content-Type", "application/json;charset=UTF-8")
    .body(Body::from(file_contents.to_owned()))
    .expect("request poi info");

  let result = HTTP_CLIENT.request(req).await;
  match &result {
    Ok(_) => {}
    Err(error) => {
      println!("{}", error);
    }
  }
}

#[cfg(test)]
pub mod tests {
  use super::*;
  #[test]
  #[allow(unused_must_use)]
  fn test1() {
    // tokio::task::spawn_blocking(move || async move {
    //   request_post().await;
    // });
    let t1 = std::thread::spawn(move || {
      println!("test~");
      // tokio::task::spawn_blocking(move || async move {
      //   request_post().await;
      // });
      // request_post().await;
    });
    t1.join();
  }
}
