use serde::Deserialize;
use envy;

#[derive(Deserialize)]
pub struct ApiConfig {
    pub api_host: String,
    pub api_port: u16,
    pub api_token: String,
}

pub fn get_api_config() -> ApiConfig {
  let vars = envy::from_env::<ApiConfig>().unwrap();
  vars
}
