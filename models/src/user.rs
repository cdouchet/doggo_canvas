use serde::{Deserialize, Serialize};
use ts_rs::TS;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone, TS)]
#[ts(export)]
pub struct User {
    pub uid: Uuid,
    pub username: String,
}
