use core::fmt::Debug;
use core::ops::DerefMut;
use redis::{
  cluster::ClusterClient, Client, ConnectionAddr, ConnectionInfo, ConnectionLike,
  RedisConnectionInfo, RedisError,
};
use std::result::Result;
use std::str::FromStr;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use crate::config::RedisConfig;

type RedisValue = String;

enum RedisClient {
  SingleClientType(redis::Client),
  ClusterClientType(redis::cluster::ClusterClient),
  UnknownClientType,
}

enum RedisClientConnection {
  SingleClientConnectionType(redis::Connection),
  ClusterClientConnectionType(redis::cluster::ClusterConnection),
  UnknownClientConnectionType,
}

pub struct RedisClientOperation {
  redis_client: Arc<RedisClient>,
  redis_client_connection: Arc<Mutex<RedisClientConnection>>,
}

impl Debug for RedisClientOperation {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("RedisClientOperation")
      .field("redis_client", &"&self.redis_client")
      .field("redis_client_connection", &"&self.redis_client_connection")
      .finish()
  }
}

impl Default for RedisClientOperation {
  fn default() -> Self {
    Self {
      redis_client: Arc::new(RedisClient::UnknownClientType),
      redis_client_connection: Arc::new(Mutex::new(
        RedisClientConnection::UnknownClientConnectionType,
      )),
    }
  }
}

impl Clone for RedisClientOperation {
  fn clone(&self) -> Self {
    Self {
      redis_client: self.redis_client.clone(),
      redis_client_connection: self.redis_client_connection.clone(),
    }
  }
}

impl RedisClientOperation {
  pub fn new(config: &RedisConfig) -> Self {
    let info = RedisClientOperation::get_redis_info(config);
    if 0 == info.len() {
      panic!("RedisManager init is failed");
    } else if 1 == info.len() {
      let client = Client::open(info[0].clone()).expect("open redis client is failed.");
      let mut conn = client.get_connection().expect("get connection is failed");
      assert_eq!(true, conn.check_connection());
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
      RedisClientOperation {
        redis_client: Arc::new(RedisClient::SingleClientType(client)),
        redis_client_connection: Arc::new(Mutex::new(
          RedisClientConnection::SingleClientConnectionType(conn),
        )),
      }
    } else {
      let client = ClusterClient::open(info).expect("open redis client is failed.");
      let mut conn = client.get_connection().expect("get connection is failed");
      assert_eq!(true, conn.check_connection());
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
      RedisClientOperation {
        redis_client: Arc::new(RedisClient::ClusterClientType(client)),
        redis_client_connection: Arc::new(Mutex::new(
          RedisClientConnection::ClusterClientConnectionType(conn),
        )),
      }
    }
  }
  pub fn set(&mut self, id: &String, value: &mut RedisValue) -> Result<(), RedisError> {
    // let mut ret = self.redis_client_connection.as_ref();
    // let mut ret = Arc::make_mut(&mut self.redis_client_connection);
    let mut grc = self.redis_client_connection.lock().expect("");
    let rcc = grc.deref_mut();
    match rcc {
      RedisClientConnection::SingleClientConnectionType(conn) => {
        let key = id;
        let value = &*value;
        redis::cmd("SET").arg(&key).arg(&value).query::<()>(conn)
      }
      RedisClientConnection::ClusterClientConnectionType(conn) => {
        let key = id;
        let value = &*value;
        redis::cmd("SET").arg(&key).arg(&value).query::<()>(conn)
      }
      RedisClientConnection::UnknownClientConnectionType => {
        panic!("Unknown client type")
      }
    }
  }
  pub fn get(&mut self, id: &String) -> Option<RedisValue> {
    let mut grc = self.redis_client_connection.lock().expect("");
    let rcc = grc.deref_mut();
    match rcc {
      RedisClientConnection::SingleClientConnectionType(conn) => {
        let key = id;
        let ret = redis::cmd("GET").arg(&key).query::<String>(conn);
        match ret {
          Ok(value) => Some(value),
          Err(error) => {
            println!("get value is error: {}", error);
            None
          }
        }
      }
      RedisClientConnection::ClusterClientConnectionType(conn) => {
        let key = id;
        let ret = redis::cmd("GET").arg(&key).query::<String>(conn);
        match ret {
          Ok(value) => Some(value),
          Err(error) => {
            println!("get value is error: {}", error);
            None
          }
        }
      }
      RedisClientConnection::UnknownClientConnectionType => {
        panic!("Unknown client type")
      }
    }
  }
  fn get_redis_info(config: &RedisConfig) -> Vec<ConnectionInfo> {
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
