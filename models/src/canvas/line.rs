use serde::{Deserialize, Serialize};
use ts_rs::TS;

use super::{coordinate::Coordinate, pencil_color::PencilColor, pencil_thickness::PencilThickness};

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct Line {
    from: Coordinate,
    to: Coordinate,
    thickness: PencilThickness,
    color: PencilColor,
}
