use super::route_wrapper::RouteWrapper;
use super::redis_client::RedisClientOperation;
use crate::config::Config;
use redis::RedisError;
use std::{result::Result, sync::Arc};

#[derive(Debug, Default)]
pub struct Service {
  redis_client: RedisClientOperation,
  route_wrapper: RouteWrapper,
}

impl Clone for Service {
  fn clone(&self) -> Self {
    Self {
      redis_client: self.redis_client.clone(),
      route_wrapper: self.route_wrapper.clone(),
    }
  }
}

impl Service {
  pub fn new(config: Arc<Config>) -> Service {
    Service {
      redis_client: RedisClientOperation::new(&config.clone().redis_config),
      route_wrapper: Default::default(),
    }
  }
  pub fn save_value(&mut self, id: &String, value: &mut String) -> Result<(), RedisError> {
    self.redis_client.set(id, value)
  }
  pub fn get_value(&mut self, id: &String) -> Option<String> {
    self.redis_client.get(id)
  }
}
