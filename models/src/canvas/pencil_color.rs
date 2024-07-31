use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[serde(rename_all = "snake_case")]
#[ts(export)]
pub enum PencilColor {
    Red,
    Blue,
    Green,
    Brown,
    Yellow,
    Pink,
    Black,
    Grey,
}
