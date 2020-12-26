use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: uuid::Uuid,
    pub username: String,
    pub email: Option<String>,
    pub password: Option<String>,
    pub created: DateTime<Utc>,
}

impl Default for User {
    fn default() -> Self {
        User {
            id: uuid::Uuid::new_v4(),
            created: Utc::now(),
            password: None,
            email: None, ..Default::default()
        }
    }
}
