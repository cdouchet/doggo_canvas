use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize)]
pub struct CursorPosition {
    pub x: u64,
    pub y: u64,
}

#[derive(Serialize)]
pub struct CursorPositionWithUuid {
    pub x: u64,
    pub y: u64,
    pub id: Uuid,
}

impl CursorPositionWithUuid {
    pub fn from_cursor_position(cursor_position: CursorPosition, uuid: Uuid) -> Self {
        Self {
            x: cursor_position.x,
            y: cursor_position.y,
            id: uuid,
        }
    }
}
