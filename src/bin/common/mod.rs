use clap::Parser;
use clap_verbosity_flag::{InfoLevel, Verbosity};
use serde::{Deserialize, Serialize};

pub static APP_NAME: &str = "App-Template";

#[derive(Debug, Serialize, Deserialize)]
pub struct MyConfig {
    pub config_version: u8,
}

/// `MyConfig` implements `Default`
impl ::std::default::Default for MyConfig {
    fn default() -> Self {
        Self { config_version: 1 }
    }
}

#[derive(Parser)]
#[clap(author, about, version)]
pub struct ArgsCli {
    #[command(flatten)]
    pub verbose: Verbosity<InfoLevel>,
}
