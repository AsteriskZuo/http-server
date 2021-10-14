use super::Config;
use std::error::Error;
use std::fmt::Display;
use std::fs::File;
use std::io::Seek;
use std::io::{Read, SeekFrom};
use std::option::Option;
use yaml_rust::Yaml;

#[derive(Debug)]
pub struct YamlError {
  code: i32,
  reason: String,
}

impl Error for YamlError {}

impl Display for YamlError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "yaml error: {:?}:{:?}", self.code, self.reason)
  }
}

pub fn read_config_from_yaml(config: &mut Config) -> Result<(), YamlError> {
  fn _parse_config(config: &mut Config, docs: &Vec<Yaml>) -> Option<()> {
    let doc = &docs[0];

    let server_port = doc["server"]["port"].as_i64()? as u16;
    let redis_mode = doc["redis"]["mode"].as_str()?;
    let password = if false == doc["redis"]["password"].is_badvalue() {
      doc["redis"]["password"].as_str()?
    } else {
      ""
    };
    let hosts = doc["redis"]["hosts"].as_str()?;
    let dial_timeout = doc["redis"]["Connect"]["DialTimeout"].as_i64()?;
    let write_timeout = doc["redis"]["Connect"]["WriteTimeout"].as_i64()?;
    let read_timeout = doc["redis"]["Connect"]["ReadTimeout"].as_i64()?;
    config.redis_config.connect.dial_timeout = dial_timeout;
    config.redis_config.connect.read_timeout = read_timeout;
    config.redis_config.connect.write_timeout = write_timeout;
    config.redis_config.hosts = String::from(hosts);
    config.redis_config.pass = String::from(password);
    config.redis_config.mode = String::from(redis_mode);
    // config.port = server_port;
    Some(())
  }
  fn _read_file() -> Result<String, Box<dyn Error>> {
    let mut file = File::open("config.yaml")?;
    let method_type = 3;
    if 1 == method_type {
      let before = file.stream_position()?;
      let content = &mut [0; 1024];
      let size = file.read(&mut content[..])?;
      let after = file.stream_position()?;
      println!("ret={}:{}:{}:{:?}", before, after, size, &content);
      Ok(String::from_utf8(content.to_vec())?)
    } else if 2 == method_type {
      let before = file.stream_position()?;
      let mut bytes = Vec::with_capacity(100);
      file.read_to_end(&mut bytes)?;
      let after = file.stream_position()?;
      println!("ret={}:{}:{:?}", before, after, &bytes);
      Ok(String::from_utf8(bytes)?)
    } else if 3 == method_type {
      let before = file.stream_position()?;
      let mut buf = String::new();
      let size = file.read_to_string(&mut buf)?;
      let after = file.stream_position()?;
      println!("ret={}:{}:{}:{:?}", before, after, size, &buf);
      Ok(buf)
    } else {
      Err(Box::new(YamlError {
        code: 4,
        reason: String::from("method type is failed."),
      }))
    }
  }
  let content = _read_file();
  match content {
    Ok(t) => {
      let ret = yaml_rust::YamlLoader::load_from_str(&t.as_str());
      match ret {
        Ok(docs) => {
          let mut config = config;
          match _parse_config(&mut config, &docs) {
            Some(_) => {}
            None => {
              return Err(YamlError {
                code: 2,
                reason: String::from("parse is failed."),
              });
            }
          }
          return Ok(());
        }
        Err(error) => {
          println!("{:?}", error);
          return Err(YamlError {
            code: 1,
            reason: error.to_string(),
          });
        }
      }
    }
    Err(error) => {
      println!("{:?}", error);
      return Err(YamlError {
        code: 3,
        reason: error.to_string(),
      });
    }
  }
}

#[cfg(test)]
mod test {
  use yaml_rust::YamlLoader;

  use super::*;
  #[test]
  fn test_yaml() {
    let mut config = Config::default();
    println!("test_yaml>ret={:?}", read_config_from_yaml(&mut config));
  }
  #[test]
  fn test_anchor() {
    let s = "
a1: &DEFAULT
  b1: 4
  b2: d
  b3: 
a2: *DEFAULT
";
    let out = YamlLoader::load_from_str(&s).unwrap();
    let doc = &out[0];
    assert_eq!(doc["a2"]["b1"].as_i64().unwrap(), 4);
    assert_eq!(doc["a2"]["b2"].as_str().unwrap(), "d");
    println!("test_anchor={}", doc["a2"]["b2"].as_str().unwrap());
    println!("test_anchor2={}", doc["a1"]["b2"].as_str().unwrap());
    // println!("test_anchor2={}", doc["a1"]["b3"].as_str().unwrap());
  }
}
