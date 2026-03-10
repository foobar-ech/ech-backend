use serde::Deserialize;

#[derive(Deserialize)]
pub struct SharedConfig {
    pub db_host: String,
    pub db_port: u16,
    pub port_posting: u16,
}
