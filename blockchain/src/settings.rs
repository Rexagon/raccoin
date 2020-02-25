use config::{Config, ConfigError, File};
use serde::Deserialize;
use std::net::SocketAddr;

use crate::block::{Block, BlockData};

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub api: ApiSettings,
    pub genesis: Block<BlockData>,
}

#[derive(Debug, Deserialize)]
pub struct ApiSettings {
    pub rpc: SocketAddr,
    pub rest: SocketAddr,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let mut s = Config::new();

        s.merge(File::with_name("settings.json"))?;

        s.try_into()
    }
}
