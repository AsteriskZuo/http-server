use std::error::Error;
use std::fmt::Display;
use std::result::Result;

#[derive(Default, Debug)]
pub struct RedisError {
  code: RedisErrorCode,
}

impl Display for RedisError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "redis error code: {}", self.code)
  }
}

impl Error for RedisError {}

type RedisErrorCode = i32;

pub fn get_result(id: &str) -> Result<&[u8], RedisError> {
  match id.is_empty() {
    true => return Ok("".as_bytes()),
    _ => return Err(RedisError{code: -1}),
  }
}
