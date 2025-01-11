// use std::time::Instant;
// #[cfg(test)]
// use tictactoe::board::Board;
// use tictactoe::coordinate::Coordinate;
// use tictactoe::coordinate::ValidCoordinate;
// use tictactoe::error::Error;
// use tictactoe::error::Result;
// use tictactoe::game::Game;
// use tictactoe::player::Player;
// use tictactoe::strategy::random::RandomStrategy;
// use tictactoe::ui::UserInterface;
// mod utils;

// struct NothingUI {}

// impl NothingUI {
//     pub fn new() -> Self {
//         NothingUI {}
//     }
// }

// impl UserInterface for NothingUI {
//     fn get_input(&self, current_player: &Player, board: &Board) -> Result<ValidCoordinate> {
//         let c = Coordinate(1, 1);
//         ValidCoordinate::from(&c, board)
//     }
//     fn display_error(&self, error: Error) {}
//     fn display_board(&self, board: &Board) {}
//     fn display_winner(&self, player: &Player) {}
//     fn display_draw(&self) {}
// }

// #[test]
// fn performance_test() {
//     let start = Instant::now();
//     for _ in 0..1000000 {
//         let nothing = NothingUI::new();
//         let mut tic_tac_toe = Game::new(nothing);
//         let random_1 = RandomStrategy::new();
//         let random_2 = RandomStrategy::new();
//         utils::play_by_itself(&mut tic_tac_toe, &random_1, &random_2);
//     }
//     let elapsed = start.elapsed();
//     panic!("Elapsed: {elapsed:?}")
// }
