use crate::models::User;
use std::path;

pub static PKGDATADIR: &str = "/app/share/pl";
pub static VERSION: &str = "0.1.0";
pub static LOCALEDIR: &str = "/app/share/locale";

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    user_credentials: User,
    date_format: String,
    data_dir: path::PathBuf,
    output_format: OutputFormat,
    theme: Theme,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Theme {
    Dark, Light, Auto,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum OutputFormat {
    JSON, YAML, TOML
}
