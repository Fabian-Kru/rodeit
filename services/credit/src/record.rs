use schemars::JsonSchema;
use serde::{
    Deserialize,
    Serialize
};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct Entry {
    // ID for Database Entry
    pub id: u32,
    // UserId
    pub userId: Uuid,
    // Timestamp, when this information got recorded
    pub timestamp: u64
}