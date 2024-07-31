use serde::{Deserialize, Serialize};
use ts_rs::TS;
use uuid::Uuid;

use super::line::Line;

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct UserLine {
    pub user_id: Uuid,
    pub line: Line,
}
