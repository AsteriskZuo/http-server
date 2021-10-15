use redis::cluster::ClusterConnection;
use redis::Cmd;
use redis::{
  cluster::ClusterClient, Client, Connection, ConnectionAddr, ConnectionInfo, ConnectionLike,
  RedisConnectionInfo, RedisError,
};
use std::error::Error;
use std::fmt::Display;
use std::result::Result;
use std::str::FromStr;
use std::time::Duration;

use crate::config::RedisConfig;

type RedisValue = String;
type RedisErrorCode = i32;

// enum ClientType {
//   SingleClient(redis::Client),
//   ClusterClient(redis::cluster::ClusterClient),
// }

// enum ClientConnectionType {
//   SingleConnection(redis::Connection),
//   ClusterConnection(redis::cluster::ClusterConnection),
// }

pub struct RedisClientManager {
  client: Box<dyn RedisClientTrait>,
  conn: Box<dyn RedisClientConnectionTrait>,
}

// #[derive(Default, Debug)]
// pub struct RedisError {
//   pub code: RedisErrorCode,
//   pub reason: String,
// }

// impl Display for RedisError {
//   fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//     write!(f, "redis error code: {}", self.code)
//   }
// }

// impl Error for RedisError {}

trait RedisClientTrait {}

impl RedisClientTrait for redis::Client {}
impl RedisClientTrait for redis::cluster::ClusterClient {}

trait RedisClientConnectionTrait {
  fn get(&mut self, key: &String) -> Option<RedisValue>;
  fn set(&mut self, key: &String, value: &RedisValue) -> Result<(), RedisError>;
}

impl RedisClientConnectionTrait for redis::Connection {
  fn get(&mut self, key: &String) -> Option<RedisValue> {
    let ret = redis::cmd("GET").arg(&key).query::<String>(self);
    match ret {
      Ok(value) => Some(value),
      Err(error) => {
        println!("get value is error: {}", error);
        None
      }
    }
  }

  fn set(&mut self, key: &String, value: &RedisValue) -> Result<(), RedisError> {
    redis::cmd("SET").arg(&key).arg(&value).query::<()>(self)
  }
}

impl RedisClientConnectionTrait for redis::cluster::ClusterConnection {
  fn get(&mut self, key: &String) -> Option<RedisValue> {
    let ret = redis::cmd("GET").arg(&key).query::<String>(self);
    match ret {
      Ok(value) => Some(value),
      Err(error) => {
        println!("get value is error: {}", error);
        None
      }
    }
  }

  fn set(&mut self, key: &String, value: &RedisValue) -> Result<(), RedisError> {
    redis::cmd("SET").arg(&key).arg(&value).query::<()>(self)
  }
}

impl RedisClientManager {
  pub fn init(&mut self, config: &RedisConfig) {
    let info = self.get_redis_info(config);
    if 0 == info.len() {
      panic!("RedisManager init is failed");
    } else if 1 == info.len() {
      let info = self.get_redis_info(&config);
      let client = Client::open(info[0].clone()).expect("open redis client is failed.");
      let mut conn = client.get_connection().expect("get connection is failed");
      conn
        .set_read_timeout(Some(Duration::from_millis(
          config.connect.read_timeout as u64,
        )))
        .expect("set read timeout is failed");
      conn
        .set_write_timeout(Some(Duration::from_millis(
          config.connect.write_timeout as u64,
        )))
        .expect("set write timeout is failed");
      ConnectionLike::check_connection(&mut conn);
      self.client = Box::new(client);
      self.conn = Box::new(conn);
    } else {
      let info = self.get_redis_info(&config);
      let client = ClusterClient::open(info).expect("open redis client is failed.");
      let mut conn = client.get_connection().expect("get connection is failed");
      conn
        .set_read_timeout(Some(Duration::from_millis(
          config.connect.read_timeout as u64,
        )))
        .expect("set read timeout is failed");
      conn
        .set_write_timeout(Some(Duration::from_millis(
          config.connect.write_timeout as u64,
        )))
        .expect("set write timeout is failed");
      ConnectionLike::check_connection(&mut conn);
      self.client = Box::new(client);
      self.conn = Box::new(conn);
    }
  }
  pub fn uninit(&mut self) {}
  pub fn set(&mut self, id: &String, value: &mut RedisValue) -> Result<(), RedisError> {
    self.conn.set(id, value)
  }
  pub fn get(&mut self, id: &String) -> Option<RedisValue> {
    self.conn.get(id)
  }

  fn get_redis_info(&self, config: &RedisConfig) -> Vec<ConnectionInfo> {
    fn _create_info(item: &str) -> ConnectionInfo {
      let sub_item = item.split(':').collect::<Vec<&str>>();
      let host = std::string::String::from(sub_item[0]);
      let port = u16::from_str(sub_item[1]).expect("parse port is error.");
      let con_addr = ConnectionAddr::Tcp(host, port);
      let con_info = RedisConnectionInfo {
        db: 0,
        username: None,
        password: None,
      };
      ConnectionInfo {
        addr: con_addr,
        redis: con_info,
      }
    }
    let mut ret = Vec::<ConnectionInfo>::new();
    let list = config.hosts.split(',').collect::<Vec<&str>>();
    if "cluster" == config.mode.as_str() {
      for item in list {
        ret.push(_create_info(item));
      }
    } else {
      for item in list {
        ret.push(_create_info(item));
        break;
      }
    }
    return ret;
  }
}
