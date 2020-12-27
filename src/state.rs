use std::path;
use crate::models::User;
use chrono::{DateTime, Utc, Weekday};
use serde::{Serialize, Deserialize};
use dirs_next::{home_dir, data_local_dir};


/// Defines the state for the app
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct State {

}

#[derive(Debug, Serialize, Deserialize)]
pub struct AppConfig {
    auth_creds: Option<User>,
    theme: Theme,
    date_format: String,
    output_format: OutputFormat,
    data_dir: path::PathBuf,
    week_start_date: Weekday,
    units: Units,
    encrypted_secret: Option<String>,
    notification_level: NotificationLevel,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            auth_creds: None,
            theme: Theme::Autodetect,
            output_format: OutputFormat::JSON,
            data_dir: data_local_dir().unwrap().join("per"),
            week_start_date: Weekday::Sun,
            units: Units::SI,
            encrypted_secret: None,
            notification_level: NotificationLevel::Important,
            date_format: "YYYY-MM-DD, HH: MM: SS".into(),
        }
    }
}

impl AppConfig {

    pub fn get_or_create_default_data_dir() -> std::io::Result<()> {
        unimplemented!()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Theme {
    Dark, Light, Autodetect
}


#[derive(Debug, Serialize, Deserialize)]
pub enum NotificationLevel {
    NoNotifications, AllNotifications, Important
}

#[derive(Debug, Serialize, Deserialize)]
pub enum UnitKind {
    SI, ImperialUK, ImperialUS
}

#[derive(Debug, Serialize, Deserialize)]
pub enum OutputFormat {
    JSON, YAML, TOML,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Units {
    SI, ImperialUk, ImperialUS,
}
