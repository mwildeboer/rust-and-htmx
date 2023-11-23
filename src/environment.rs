use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Environment {
    #[serde(default = "default_log_level")]
    pub log_level: String,

    #[serde(default = "default_port")]
    pub port: u16,

    #[serde(default = "default_db_port")]
    pub db_port: u16,

    #[serde(default = "default_db_user")]
    pub db_user: String,

    #[serde(default = "default_db_password")]
    pub db_password: String,

    #[serde(default = "default_db_name")]
    pub db_name: String,
}

fn default_log_level() -> String {
    "debug".to_string()
}

fn default_port() -> u16 {
    8000
}

fn default_db_port() -> u16 {
    5432
}

fn default_db_user() -> String {
    "postgres".to_string()
}

fn default_db_password() -> String {
    "secret".to_string()
}

fn default_db_name() -> String {
    "tests".to_string()
}
