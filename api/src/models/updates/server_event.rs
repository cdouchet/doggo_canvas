use models::{
    canvas::{coordinate::Coordinate, user_line::UserLine},
    user::User,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use warp::filters::ws::Message;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ServerEvent {
    UserJoin(User),
    UserLeft(Uuid),
    UserCursorPositionUpdate {
        user_id: Uuid,
        coordinate: Coordinate,
    },
    DrawLine(UserLine),
}

impl ServerEvent {
    pub fn to_websocket_message(&self) -> Message {
        Message::text(serde_json::to_string(&self).unwrap())
    }
}
