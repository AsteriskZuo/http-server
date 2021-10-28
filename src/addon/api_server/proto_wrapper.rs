#![allow(non_snake_case)]

use crate::protos::{
  route_client_param::RoutePlanClientParameter,
  route_common::GeoPoint,
  route_server_param::{
    RoutePlanServerParameter, RoutePlanServerParameter_oneof_end,
    RoutePlanServerParameter_oneof_start,
  },
};
use protobuf::Message;
use std::error::Error;

use super::service;

pub async fn client_to_server_protobuf(
  data: &String,
  services: &service::Service,
) -> Result<Vec<u8>, Box<dyn Error>> {
  let client_params = RoutePlanClientParameter::parse_from_bytes(data.as_bytes()).unwrap();
  let start_point_info = services.get_poi_info(&client_params.startPoiID).await;
  let end_point_info = services.get_poi_info(&client_params.endPoiID).await;
  let mut server_params = RoutePlanServerParameter::new();
  server_params.start = Option::Some(RoutePlanServerParameter_oneof_start::startPoi(
    start_point_info.expect("start_point_info"),
  ));
  server_params.end = Option::Some(RoutePlanServerParameter_oneof_end::endPoi(
    end_point_info.expect("end_point_info"),
  ));
  server_params.version = client_params.version;
  server_params.mode = client_params.mode;
  server_params.policy = client_params.policy;
  server_params.realTimeTraffic = client_params.realTimeTraffic;
  let server_params_bytes = server_params.write_to_bytes().expect("server_params_bytes");
  // let ret = String::from_utf8(server_params_bytes).expect("server_params_bytes");
  return Ok(server_params_bytes);
}

pub async fn client_json_to_server_protobuf(
  data: &String,
  services: &service::Service,
) -> Result<Vec<u8>, Box<dyn Error>> {
  let json_object =
    serde_json::from_str::<serde_json::value::Value>(data.as_str()).expect("json_object");
  let version = json_object["version"].as_u64().expect("version") as u32;
  let mode = if let Option::Some(opt) = json_object["mode"].as_u64() {
    opt as u32
  } else {
    0
  };
  let policy = if let Option::Some(opt) = json_object["policy"].as_u64() {
    opt as u32
  } else {
    0
  };
  let realTimeTraffic = if let Option::Some(opt) = json_object["realTimeTraffic"].as_bool() {
    opt
  } else {
    false
  };
  let startPoiID = if let Option::Some(opt) = json_object["startPoiID"].as_str() {
    opt
  } else {
    ""
  };
  let endPoiID = if let Option::Some(opt) = json_object["endPoiID"].as_str() {
    opt
  } else {
    ""
  };

  let mut server_params = RoutePlanServerParameter::new();
  if "" != startPoiID {
    let start_point_info = services.get_poi_info(&String::from(startPoiID)).await;
    server_params.start = Option::Some(RoutePlanServerParameter_oneof_start::startPoi(
      start_point_info.expect("start_point_info"),
    ));
  } else {
    let mut startPoint = GeoPoint::new();
    startPoint.longitude = json_object["startPoint"]["longitude"]
      .as_f64()
      .expect("longitude");
    startPoint.latitude = json_object["startPoint"]["latitude"]
      .as_f64()
      .expect("latitude");
    startPoint.height = json_object["startPoint"]["height"]
      .as_i64()
      .expect("height") as i32;
    startPoint.floor = if let Option::Some(opt) = json_object["startPoint"]["floor"].as_i64() {
      opt as i32
    } else {
      0
    };
    startPoint.modelID = if let Option::Some(opt) = json_object["startPoint"]["modelId"].as_u64() {
      opt as u32
    } else {
      0
    };
    server_params.start =
      Option::Some(RoutePlanServerParameter_oneof_start::startPoint(startPoint));
  }
  if "" != endPoiID {
    let end_point_info = services.get_poi_info(&String::from(endPoiID)).await;
    server_params.end = Option::Some(RoutePlanServerParameter_oneof_end::endPoi(
      end_point_info.expect("end_point_info"),
    ));
  } else {
    let mut endPoint = GeoPoint::new();
    endPoint.longitude = json_object["endPoint"]["longitude"]
      .as_f64()
      .expect("longitude");
    endPoint.latitude = json_object["endPoint"]["latitude"]
      .as_f64()
      .expect("latitude");
    endPoint.height = json_object["endPoint"]["height"].as_i64().expect("height") as i32;
    endPoint.floor = if let Option::Some(opt) = json_object["endPoint"]["floor"].as_i64() {
      opt as i32
    } else {
      0
    };
    endPoint.modelID = if let Option::Some(opt) = json_object["endPoint"]["modelId"].as_u64() {
      opt as u32
    } else {
      0
    };
    server_params.end = Option::Some(RoutePlanServerParameter_oneof_end::endPoint(endPoint));
  }

  server_params.version = version;
  server_params.mode = mode;
  server_params.policy = policy;
  server_params.realTimeTraffic = realTimeTraffic;
  let server_params_bytes = server_params.write_to_bytes().expect("server_params_bytes");
  // let ret = String::from_utf8(server_params_bytes).expect("server_params_bytes");
  return Ok(server_params_bytes);
}
