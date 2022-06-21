use serde::Deserialize;
use ::config::Config;
use dotenv::dotenv;

#[derive(Debug, Default, Deserialize)]
pub struct ExampleConfig {
    pub server_addr: String,
    pub pg: deadpool_postgres::Config,
}

impl ExampleConfig {
    pub fn new() -> Self {
        dotenv().ok();
        let config = Config::builder()
            .add_source(::config::Environment::default())
            .build()
            .unwrap();
        config.try_deserialize().unwrap()
    }
}
