use anyhow::{Error, Result};
use http::{HeaderValue, Method, Uri};
use serde::Deserialize;
use std::convert::TryFrom;

use crate::addon::proxy::Proxy;

#[derive(Clone, Debug, Deserialize)]
pub struct ProxyConfig {
    pub is_dynamic: bool,
    pub url: Option<String>,
    pub method: Option<String>,
    pub authorization: Option<String>,
}

impl ProxyConfig {
    pub fn new_dynamic() -> Self {
        ProxyConfig {
            is_dynamic: true,
            url: None,
            method: None,
            authorization: None,
        }
    }

    pub fn raw_url(url: String) -> Self {
        ProxyConfig {
            is_dynamic: false,
            url: Some(url),
            method: Some(String::from("GET")),
            authorization: None,
        }
    }
}

impl TryFrom<ProxyConfig> for Proxy {
    type Error = Error;

    fn try_from(value: ProxyConfig) -> Result<Self, Self::Error> {
        if value.is_dynamic {
            return Ok(Proxy::new_dynamic());
        }

        if value.url.is_some() && value.method.is_some() && value.authorization.is_some() {
            let url: Uri = value.url.unwrap().parse().map_err(Error::from)?;
            let method: Method = value.method.unwrap().parse().map_err(Error::from)?;

            if let Some(authorization) = value.authorization {
                let auth_header =
                    HeaderValue::from_str(authorization.as_str()).map_err(Error::from)?;

                return Ok(Proxy::new_static(url, method, Some(auth_header)));
            }

            return Ok(Proxy::new_static(url, method, None));
        }

        Err(Error::msg("Invalid configuration for Proxy provided"))
    }
}
