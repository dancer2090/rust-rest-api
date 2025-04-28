use serde::Deserialize;

#[derive(Deserialize)]
pub struct ApiConfig {
    pub api_host: String,
    pub api_port: u16,
}

