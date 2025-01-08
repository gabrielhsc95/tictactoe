pub mod tui;
use crate::board::Board;
use crate::coordinate::Coordinate;
use crate::error::Error;
use crate::error::Result;
use crate::player::Player;

pub trait UserInterface {
    fn get_input(&self, current_player: &Player, board: &Board) -> Result<Coordinate>;
    fn display_error(&self, error: Error);
    fn display_board(&self, board: &Board);
    fn display_winner(&self, player: &Player);
    fn display_draw(&self);
}
