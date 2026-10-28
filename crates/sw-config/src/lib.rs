use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use sw_errors::{Result, WeaveError};
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Config {
    pub port: u16,
    pub data_dir: PathBuf,
    pub request_timeout_ms: u64,
    pub max_body_bytes: usize,
}
impl Config {
    pub fn from_env() -> Result<Self> {
        let port = std::env::var("PORT")
            .unwrap_or_else(|_| "8080".into())
            .parse()
            .map_err(|_| WeaveError::Invalid("PORT".into()))?;
        Ok(Self {
            port,
            data_dir: std::env::var("SUPPLY_WEAVE_DATA")
                .unwrap_or_else(|_| "data/runtime".into())
                .into(),
            request_timeout_ms: 5000,
            max_body_bytes: 65536,
        })
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn defaults_are_bounded() {
        let c = Config::from_env().unwrap();
        assert!(c.request_timeout_ms < 10000);
    }
}
