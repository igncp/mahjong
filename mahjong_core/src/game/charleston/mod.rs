use super::Game;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Clone, Serialize, Deserialize, Debug, TS)]
#[ts(export)]
pub struct Charleston {}

pub enum MoveCharlestonError {
    CharlestonAlreadyDone,
}

impl Game {
    pub fn move_charleston(&self) -> Result<(), MoveCharlestonError> {
        // if !self.charleston.is_some() {
        //     return Err(MoveCharlestonError::CharlestonAlreadyDone);
        // }

        Ok(())
    }
}
