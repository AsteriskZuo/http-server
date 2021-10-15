use std::result::Result;

use redis::RedisError;

// use lazy_static::lazy_static;

// lazy_static! {
//   static ref SERVICES: Client<HttpConnector> = Client::new();
// }

struct services {

}

pub fn get_result(id: &str) -> Result<&[u8], RedisError> {
  todo!()
  // redis::get_result(id)
  // match !id.is_empty() {
  //   true => return Ok("123".as_bytes()),
  //   Err(error) => Err(error)
  // }
}
