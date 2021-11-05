#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct ServerConfig {
    pub ip: String,
    pub port: usize,
    pub enable: bool,
}
