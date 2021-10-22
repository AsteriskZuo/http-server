use crate::protos::{
  route_client_param::RoutePlanClientParameter,
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
) -> Result<String, Box<dyn Error>> {
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
  let ret = String::from_utf8(server_params_bytes).expect("server_params_bytes");
  return Ok(ret);
}
