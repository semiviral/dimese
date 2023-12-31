use once_cell::sync::Lazy;
use serde::Deserialize;
use std::{net::SocketAddr, path::PathBuf};

#[derive(Debug, Deserialize)]
pub struct Cfg {
    pub server: Server,
    pub storage: Storage,
}

#[derive(Debug, Deserialize)]
pub struct Server {
    pub address: SocketAddr,
}

#[derive(Debug, Deserialize)]
pub struct Storage {
    pub directory: PathBuf,
    pub max: u64,
}

pub fn get() -> &'static Cfg {
    static APP_CONFIG: Lazy<Cfg> = Lazy::new(|| {
        use config::{Config, Environment};

        Config::builder()
            .add_source(
                Environment::with_prefix("DIMESE")
                    .separator("_")
                    .list_separator(","),
            )
            .build()
            .and_then(Config::try_deserialize)
            .expect("failed to read configuration")
    });

    &APP_CONFIG
}
