use super::Game;

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
