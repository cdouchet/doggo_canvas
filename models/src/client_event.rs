use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::canvas::{coordinate::Coordinate, line::Line};

#[derive(Debug, Serialize, Deserialize, TS)]
#[serde(rename_all = "snake_case")]
#[ts(export)]
pub enum ClientEvent {
    CursorPosition(Coordinate),
    DrawLine(Line),
    UsernameUpdate(String),
}
