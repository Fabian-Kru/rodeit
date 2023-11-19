use chrono::{
    DateTime,
    Utc
};
use schemars::JsonSchema;
use serde::{
    Deserialize,
    Serialize
};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct Entry {
    // ID for Database Entry
    pub id: Uuid,
    // UserId
    pub userId: Uuid,
    // Timestamp, when this information got recorded
    pub timestamp: DateTime<Utc>
}

impl Entry {
    pub fn new(userId: Uuid) -> Self {
        Self {
            id: Uuid::uuid(),
            userId,
            timestamp: Utc::now(),
        }
    }

    pub fn load(id: Uuid, userId: Uuid, timestamp: DateTime<Utc>) -> Self {
        Self {
            id,
            userId,
            timestamp
        }
    }
}
