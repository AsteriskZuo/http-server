use std::{result::Result, sync::Arc};

use redis::RedisError;

use crate::config::Config;

use super::redis_client::RedisClientOperation;

#[derive(Debug, Default)]
pub struct Service {
  redis_client: RedisClientOperation,
}

impl Clone for Service {
    fn clone(&self) -> Self {
        Self { redis_client: self.redis_client.clone() }
    }
}

impl Service {
  pub fn new(config: Arc<Config>) -> Service {
    Service {
      redis_client: RedisClientOperation::new(&config.clone().redis_config),
    }
  }
  pub fn save_value(&mut self, id: &String, value: &mut String) -> Result<(), RedisError> {
    todo!()
  }
  pub fn get_value(&mut self, id: &String) -> Option<String> {
    self.redis_client.get(id)
  }
}

pub fn get_result(id: &str) -> Result<&[u8], RedisError> {
  todo!()
  // redis::get_result(id)
  // match !id.is_empty() {
  //   true => return Ok("123".as_bytes()),
  //   Err(error) => Err(error)
  // }
}
