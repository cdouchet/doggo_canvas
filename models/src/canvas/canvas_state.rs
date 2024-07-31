use serde::{Deserialize, Serialize};
use ts_rs::TS;

use super::user_line::UserLine;

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct CanvasState(Vec<UserLine>);

impl CanvasState {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn add_user_line(&mut self, user_line: UserLine) {
        self.0.push(user_line);
    }

    pub fn get_memory_size(&self) -> usize {
        std::mem::size_of_val(&*self.0)
    }
}
