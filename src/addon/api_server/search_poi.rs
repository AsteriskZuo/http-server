// https://github.com/rust-lang/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![allow(unused_attributes)]
#![cfg_attr(rustfmt, rustfmt::skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unused_imports)]
#![allow(unused_results)]

use futures::Future;
use http::{Response, StatusCode};
use hyper::body::HttpBody;
use hyper::client::ResponseFuture;
use hyper::{Body, Client, Method, Request};
use serde_json::{Result as JsonResult, Value, json};
use std::error::Error;
use std::fmt::Display;
use std::result::Result;
use hyper::body::Bytes;
use serde::Deserialize;



const get_poi_detail: &str = "getPoiDetailByPoiId";

#[derive(Deserialize, Debug)]
pub struct PoiDetail  {
	pub addressFloor :String, //`json:"addressFloor"`
	pub buildFlag    :String, //`json:"buildFlag"`
	pub latitude     :String, //`json:"latitude"`
	pub longitude    :String, //`json:"longitude"`
	pub modelId      :String, //`json:"modelId"`
	pub parentPoiId  :String, //`json:"parentPoiId"`
	pub poiId        :String, //`json:"poiId"`
	pub poiName      :String, //`json:"poiName"`
	pub poiType      :String, //`json:"poiType"`
	pub roadId       :String, //`json:"roadId"`
	pub roadXEntr    :String, //`json:"roadXEntr"`
	pub roadYEntr    :String, //`json:"roadYEntr"`
	pub wroadId      :String, //`json:"wroadId"`
	pub wroadXEntr   :String, //`json:"wroadXEntr"`
	pub wroadYEntr   :String, //`json:"wroadYEntr"`
}
#[derive(Deserialize, Debug)]
pub struct PoiDetailResponseBody {
	pub data :PoiDetail, //`json:"data"`
}
#[derive(Deserialize, Debug)]
pub struct PoiDetailResponse  {
	pub rtnCode :String, //`json:"rtnCode"`
	pub traceId :String, //`json:"traceId"`
	pub body :PoiDetailResponseBody, //`json:"body"`
}

#[derive(Debug)]
pub struct SearchError {
  pub code: i32,
}

impl Display for SearchError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "SearchPoiInfo : error: {}", self.code)
  }
}

impl Error for SearchError {}

#[derive(Debug, Clone, Default)]
pub struct SearchPoiInfo {
  pub url: String,
}

impl SearchPoiInfo {
  pub async fn search_poi_info(&self, id: &String) -> Result<PoiDetail, SearchError> {
    let client= Client::new();
    let mut json = String::from("{\"data\": { \"poiId\":\"");
    json += id;
    json += "\"}}";
    let req = Request::builder()
      .method(Method::POST)
      .uri(self.url.clone() + get_poi_detail)
      .header("Content-Type", "application/json;charset=UTF-8")
      .body(Body::from(json))
      .expect("request poi info");

    let result = client.request(req).await;
    match result {
      Ok(mut response) => {
        let status = response.status();
        println!("search_poi_info->{}", status);
        if StatusCode::OK != status {
          println!("search_poi_info->{:?}", status);
          return Err(SearchError { code: 1 });
        }
        let orb = response.body_mut().data().await;
        match orb {
          Some(result) =>{
            match result {
              Ok(bytes) =>{
                let json = String::from_utf8(bytes.to_vec()).expect("json");
                let json_object = serde_json::from_str::<serde_json::value::Value>(json.as_str()).expect("json_object");
                let poiDetailResponse = serde_json::from_value::<PoiDetailResponse>(json_object).expect("msg");
                let data = poiDetailResponse.body.data;
                let poiName = &data.poiName;
                println!("search_poi_info->{:?}", poiName);
                return Ok(data);
              }
              Err(error)=>{
                println!("search_poi_info->{:?}", error);
        return Err(SearchError { code: 1 });
              }
            }
          }, 
          None =>{
            println!("search_poi_info->None");
          return Err(SearchError { code: 1 });
          }
        }
      }
      Err(error) => {
        println!("search_poi_info->{:?}", error);
        return Err(SearchError { code: 1 });
      }
    }
  }
}

#[cfg(test)]
pub mod tests {
  use tokio::task;
  #[test]
  fn test_url_post() {
    use super::*;
    let info = SearchPoiInfo {
      url: String::from("http://httpbin.org"),
    };
    task::spawn_blocking(move || {
      // let local_info = info.clone();
      async move {
        let ret =info.search_poi_info(&String::from("id")).await;  
        match ret {
          Ok(_) =>{
            println!("ok");
          },
          Err(error) =>{
            println!("{}", error);
          },
        }
      }      
    });    
  }
  #[test]
  fn test_json() {
    let id = "21";
    let mut json = String::from("{\"data\": ");
    json += "{ \"poiId\":\"";
    json += id;
    json += "\"}}";
    println!("test_json->{}", json);
  }
}