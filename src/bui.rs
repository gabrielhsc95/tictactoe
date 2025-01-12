pub mod btui;
use crate::bcoodinate::ValidBinaryCoordinate;
use crate::error::Error;
use crate::error::Result;

pub trait BinaryUserInterface {
    fn get_input(&self, current_player: bool, board: u32) -> Result<ValidBinaryCoordinate>;
    fn display_error(&self, error: Error);
    fn display_board(&self, board: u32);
    fn display_winner(&self, player: bool);
    fn display_draw(&self);
}
