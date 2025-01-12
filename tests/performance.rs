#[cfg(test)]
use std::time::Instant;
use tictactoe::bcoodinate::ValidBinaryCoordinate;
use tictactoe::bgame::BinaryGame;
use tictactoe::bstrategy::brandom::BinaryRandomStrategy;
use tictactoe::bui::BinaryUserInterface;
use tictactoe::error::Error;
use tictactoe::error::Result;
mod utils;

struct NothingUI {}

impl NothingUI {
    pub fn new() -> Self {
        NothingUI {}
    }
}

impl BinaryUserInterface for NothingUI {
    fn get_input(&self, current_player: bool, board: u32) -> Result<ValidBinaryCoordinate> {
        ValidBinaryCoordinate::new(0, current_player, board)
    }
    fn display_error(&self, error: Error) {}
    fn display_board(&self, board: u32) {}
    fn display_winner(&self, player: bool) {}
    fn display_draw(&self) {}
}

#[test]
fn performance_test() {
    let start = Instant::now();
    for _ in 0..1000000 {
        let nothing = NothingUI::new();
        let mut tic_tac_toe = BinaryGame::new(nothing);
        let random_1 = BinaryRandomStrategy::new();
        let random_2 = BinaryRandomStrategy::new();
        utils::play_by_itself_binary(&mut tic_tac_toe, &random_1, &random_2);
    }
    let elapsed = start.elapsed();
    panic!("Elapsed: {elapsed:?}")
}
