pub mod tui;
use crate::board::Board;
use crate::coordinate::Coordinate;
use crate::player::Player;

pub trait UserInterface {
    fn get_input(&self, current_player: &Player, board: &Board) -> Coordinate;
    fn display_board(&self, board: &Board);
    fn display_winner(&self, player: &Player);
    fn display_draw(&self);
}
