use models::user::User;
use serde::Serialize;

use super::{cursor::CursorPositionWithUuid, updates::username_update::UsernameUpdate};

#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CanvasMessage {
    CursorPosition(CursorPositionWithUuid),
    UserJoin(User),
    UserLeft(User),
    UsernameUpdate(UsernameUpdate),
}
