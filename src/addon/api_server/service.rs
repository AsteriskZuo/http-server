use super::proto_wrapper;
use super::redis_client::RedisClientOperation;
use super::route_wrapper::RouteError;
use super::search_poi::SearchPoiInfo;
use super::{route_wrapper::RouteWrapper, search_poi::SearchError};
use crate::config::Config;
use crate::protos::route_common::GeoPoint;
use crate::protos::route_server_param::PoiInfo;
use protobuf::SingularPtrField;
use std::{result::Result, sync::Arc};

#[derive(Debug, Default)]
pub struct Service {
  redis_client: RedisClientOperation,
  poi_info: SearchPoiInfo,
}

impl Clone for Service {
  fn clone(&self) -> Self {
    Self {
      redis_client: self.redis_client.clone(),
      poi_info: self.poi_info.clone(),
    }
  }
}

impl Service {
  pub fn new(config: Arc<Config>) -> Service {
    RouteWrapper::init(String::from("routinglib"));
    Service {
      redis_client: RedisClientOperation::new(&config.clone().redis_config),
      poi_info: SearchPoiInfo {
        url: config.poi_server(),
      },
    }
  }
  // pub fn save_value(&mut self, id: &String, value: &String) -> Result<(), RedisError> {
  //   self.redis_client.set(id, value)
  // }
  pub fn get_value(&mut self, id: &String) -> Option<String> {
    self.redis_client.get(id)
  }
  pub async fn get_poi_info(&self, id: &String) -> Result<PoiInfo, SearchError> {
    let poi_info = self.poi_info.search_poi_info(id).await;
    match poi_info {
      Ok(info) => {
        fn from_str_radix(value: &String) -> f64 {
          let s = value.as_bytes();
          let s_len = value.len();
          let mut sss: [u8; 8] = [0; 8];
          for i in 0..=s_len {
            if 8 == i {
              break;
            }
            sss[i] = s[i];
          }
          return f64::from_be_bytes(sss);
        }
        let mut ret = PoiInfo::new();
        ret.poiID = u64::from_str_radix(info.poiId.as_str(), 10).expect("poiID");
        ret.poiName = info.poiName;
        ret.roadID = u64::from_str_radix(info.roadId.as_str(), 10).expect("roadID");
        let mut point = GeoPoint::new();
        point.latitude = from_str_radix(&info.latitude);
        point.longitude = from_str_radix(&info.longitude);
        point.height = 0;
        point.modelID = u32::from_str_radix(info.modelId.as_str(), 10).expect("modelID");
        point.floor = i32::from_str_radix(info.addressFloor.as_str(), 10).expect("floor");
        ret.entry = SingularPtrField::from_option(Some(point));
        ret.modelID = u64::from_str_radix(info.modelId.as_str(), 10).expect("modelID");
        return Ok(ret);
      }
      Err(error) => {
        println!("get_poi_info->{}", error);
        return Err(SearchError { code: error.code });
      }
    }
  }
  pub async fn find_path(&self, data: String) -> Result<(String, String), RouteError> {
    let decode_data = proto_wrapper::client_to_server_protobuf(&data, &self).await;
    match decode_data {
      Ok(condition) =>{
         match RouteWrapper::find_path(condition) {
           Ok(path)=> {
            return Ok(path);
           }, 
           Err(error) =>{
              return Err(RouteError { code: error.code});
           }
         }
      }
      Err(error)=>{
        println!("{}", error);
        return Err(RouteError { code: 1});
      }
    }
    // RouteWrapper::find_path(decode_data.expect("msg"))
  }
  pub async fn find_path_from_json(&self, data: String) -> Result<(String, String), RouteError> {
    let decode_data = proto_wrapper::client_json_to_server_protobuf(&data, &self).await;
    match decode_data {
      Ok(condition) =>{
         match RouteWrapper::find_path(condition) {
           Ok(path)=> {
            return Ok(path);
           }, 
           Err(error) =>{
              return Err(RouteError { code: error.code});
           }
         }
      }
      Err(error)=>{
        println!("{}", error);
        return Err(RouteError { code: 1});
      }
    }
    // RouteWrapper::find_path(decode_data.expect("msg"))
  }
}
