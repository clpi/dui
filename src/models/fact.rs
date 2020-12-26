use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Fact {
    id: uuid::Uuid,
    name: String,
    created: DateTime<Utc>,
}
