pub mod basic_auth;
pub mod compression;
pub mod cors;
pub mod file;
pub mod tls;
pub mod util;
mod yaml;

use anyhow::{Error, Result};
use std::convert::TryFrom;
use std::env::current_dir;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::path::PathBuf;

use crate::cli::Cli;

use self::basic_auth::BasicAuthConfig;
use self::compression::CompressionConfig;
use self::cors::CorsConfig;
use self::file::ConfigFile;
use self::tls::TlsConfig;

#[derive(Clone, Debug)]
pub enum ServerType {
    UnknownServices = -1,
    FileServices = 0,
    ApiServices = 1,
}

impl From<i32> for ServerType {
    fn from(v: i32) -> Self {
        match v {
            0 => ServerType::FileServices,
            1 => ServerType::ApiServices,
            _ => ServerType::UnknownServices,
        }
    }
}

// type RedisConfig struct {
// 	Pass    string       `yaml:"pass"`
// 	Mode    string       `yaml:"mode"`
// 	Hosts   string       `yaml:"hosts"`
// 	Connect RedisConnect `yaml:"connect"`
// 	Pool    RedisPool    `yaml:"pool"`
// }

// type RedisConnect struct {
// 	DialTimeout  time.Duration `yaml:"dialTimeout"`
// 	WriteTimeout time.Duration `yaml:"writeTimeout"`
// 	ReadTimeout  time.Duration `yaml:"readTimeout"`
// }

#[derive(Debug, Clone)]
pub struct RedisConnect {
    pub dial_timeout: i64,
    pub write_timeout: i64,
    pub read_timeout: i64,
}

impl RedisConnect {}

impl Default for RedisConnect {
    fn default() -> Self {
        Self {
            dial_timeout: Default::default(),
            write_timeout: Default::default(),
            read_timeout: Default::default(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct RedisConfig {
    pub pass: String,
    pub mode: String,
    pub hosts: String,
    pub connect: RedisConnect,
    pub pool: String,
}

impl RedisConfig {}

impl Default for RedisConfig {
    fn default() -> Self {
        Self {
            pass: Default::default(),
            mode: Default::default(),
            hosts: Default::default(),
            connect: Default::default(),
            pool: Default::default(),
        }
    }
}

/// Server instance configuration used on initialization
#[derive(Debug)]
pub struct Config {
    pub address: SocketAddr,
    pub host: IpAddr,
    pub port: u16,
    pub root_dir: PathBuf,
    pub verbose: bool,
    pub tls: Option<TlsConfig>,
    pub cors: Option<CorsConfig>,
    pub compression: Option<CompressionConfig>,
    pub basic_auth: Option<BasicAuthConfig>,
    pub action: ServerType,
    pub redis_config: RedisConfig,
    pub poi_server: String,
}

impl Config {
    pub fn host(&self) -> IpAddr {
        self.host
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub fn address(&self) -> SocketAddr {
        self.address
    }

    pub fn root_dir(&self) -> PathBuf {
        self.root_dir.clone()
    }

    pub fn verbose(&self) -> bool {
        self.verbose
    }

    pub fn tls(&self) -> Option<TlsConfig> {
        self.tls.clone()
    }

    pub fn cors(&self) -> Option<CorsConfig> {
        self.cors.clone()
    }

    pub fn compression(&self) -> Option<CompressionConfig> {
        self.compression.clone()
    }

    pub fn basic_auth(&self) -> Option<BasicAuthConfig> {
        self.basic_auth.clone()
    }

    pub fn action(&self) -> ServerType {
        self.action.clone()
    }

    pub fn redis(&self) -> RedisConfig {
        self.redis_config.clone()
    }
    
    pub fn init_redis(&mut self) {
        match yaml::read_config_from_yaml(self) {
            Ok(_) => (),
            Err(error) => panic!("error: {:?}", error.to_string()),
        }
    }
    pub fn poi_server(&self) -> String {
        self.poi_server.clone()
    }
}

impl Default for Config {
    fn default() -> Self {
        let host = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
        let port = 7878;
        let address = SocketAddr::new(host, port);
        let root_dir = current_dir().unwrap();

        let mut ret = Self {
            host,
            port,
            address,
            root_dir,
            verbose: false,
            tls: None,
            cors: None,
            compression: None,
            basic_auth: None,
            action: ServerType::FileServices,
            redis_config: Default::default(),
            poi_server: Default::default(),
        };
        ret.init_redis();
        return ret;
    }
}

impl TryFrom<Cli> for Config {
    type Error = anyhow::Error;

    fn try_from(cli_arguments: Cli) -> Result<Self, Self::Error> {
        let verbose = cli_arguments.verbose;
        let root_dir = if cli_arguments.root_dir.to_str().unwrap() == "./" {
            current_dir().unwrap()
        } else {
            cli_arguments.root_dir.canonicalize().unwrap()
        };

        let tls: Option<TlsConfig> = if cli_arguments.tls {
            Some(TlsConfig::new(
                cli_arguments.tls_cert,
                cli_arguments.tls_key,
                cli_arguments.tls_key_algorithm,
            )?)
        } else {
            None
        };

        let cors: Option<CorsConfig> = if cli_arguments.cors {
            // when CORS is specified from CLI the default
            // configuration should allow any origin, method and
            // headers
            Some(CorsConfig::allow_all())
        } else {
            None
        };

        let compression: Option<CompressionConfig> = if cli_arguments.gzip {
            Some(CompressionConfig { gzip: true })
        } else {
            None
        };

        let basic_auth: Option<BasicAuthConfig> =
            if cli_arguments.username.is_some() && cli_arguments.password.is_some() {
                Some(BasicAuthConfig::new(
                    cli_arguments.username.unwrap(),
                    cli_arguments.password.unwrap(),
                ))
            } else {
                None
            };
        match cli_arguments.server_type {
            0 => (),
            1 => (),
            _ => panic!("not support this type: {}", cli_arguments.server_type),
        }

        let mut ret = Config {
            host: cli_arguments.host,
            port: cli_arguments.port,
            address: SocketAddr::new(cli_arguments.host, cli_arguments.port),
            root_dir,
            verbose,
            tls,
            cors,
            compression,
            basic_auth,
            action: ServerType::from(cli_arguments.server_type),
            redis_config: Default::default(),
            poi_server: Default::default(),
        };
        ret.init_redis();
        return Ok(ret);
    }
}

impl TryFrom<ConfigFile> for Config {
    type Error = Error;

    fn try_from(file: ConfigFile) -> Result<Self, Self::Error> {
        let root_dir = file.root_dir.unwrap_or_default();
        let verbose = file.verbose.unwrap_or(false);
        let tls: Option<TlsConfig> = if let Some(https_config) = file.tls {
            Some(TlsConfig::new(
                https_config.cert,
                https_config.key,
                https_config.key_algorithm,
            )?)
        } else {
            None
        };

        let mut ret = Config {
            host: file.host,
            port: file.port,
            address: SocketAddr::new(file.host, file.port),
            verbose,
            root_dir,
            tls,
            cors: file.cors,
            compression: file.compression,
            basic_auth: file.basic_auth,
            action: ServerType::FileServices,
            redis_config: Default::default(),
            poi_server: Default::default(),
        };
        ret.init_redis();
        return Ok(ret);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_default_config() {
        let host = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
        let port = 7878;
        let address = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 7878);
        let config = Config::default();

        assert_eq!(
            config.host,
            IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
            "default host: {}",
            host
        );
        assert_eq!(config.port, 7878, "default port: {}", port);
        assert_eq!(
            config.address, address,
            "default socket address: {}",
            address
        );
        assert!(!config.verbose, "verbose is off by default");
    }
}
