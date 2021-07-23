use serde::Deserialize;

use crate::addon::proxy::Kind;

pub enum ConfigKind {
    // impl a config kind as a DTO for the Kind perse
}

#[derive(Clone, Debug, Deserialize)]
pub struct ProxyConfig {
    pub kind: Kind,
}
