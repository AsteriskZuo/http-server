use http::{Method, Request};
use hyper::client::HttpConnector;
use hyper::{Body, Client};
use lazy_static::lazy_static;

#[tokio::main]
pub async fn main() {
  return multi_thread_tasks().await;
}


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
    .uri("http://127.0.0.1:8080/api/v1/navijson")
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

async fn concurrent_request_post() {
  for _ in 0..1 {
    request_post().await
  }
}

async fn multi_thread_tasks() {
  let mut list = vec![];
  for _ in 0..10 {
    list.push(std::thread::spawn(move || {
      return concurrent_request_post;
      // let ret = tokio::spawn(async {
      //   concurrent_request_post().await;
      // });
      // return ret;
    }));
  }
  // ref: https://www.koderhq.com/tutorial/rust/concurrency/
  for i in list {
    match i.join() {
      Ok(future) =>{
        println!("test:{:?}", future().await);
      }
      Err(error)=>{
        println!("{:?}", error);
      }
    }
  }
}