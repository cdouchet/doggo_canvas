use serde::{Deserialize, Serialize};
use ts_rs::TS;
use uuid::Uuid;

use crate::{canvas::canvas_state::CanvasState, user::User};

#[derive(Debug, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct Room {
    pub uid: Uuid,
    pub users: Vec<User>,
    pub public: bool,
    pub state: CanvasState,
}
