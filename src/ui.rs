pub mod tui;
use crate::board::Board;
use crate::coordinates::Coordinates;
use crate::player::Player;

pub trait UserInterface {
    fn get_input(&self, current_player: &Player, board: &Board) -> Coordinates;
    fn display_board(&self, board: &Board);
    fn display_winner(&self, player: &Player);
    fn display_draw(&self);
}
