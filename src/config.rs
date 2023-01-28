use confy::ConfyError;
use serde::{Deserialize, Serialize};
use std::env;

const APP_NAME: &str = env!("CARGO_PKG_NAME");

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Config {
    pub token: Option<String>,
}

impl Config {
    pub fn load() -> Result<Self, ConfyError> {
        confy::load(APP_NAME, None)
    }

    pub fn store(&self) -> Result<(), ConfyError> {
        confy::store(APP_NAME, None, self)
    }
}
