use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct Coordinate {
    pub x: f32,
    pub y: f32,
}
