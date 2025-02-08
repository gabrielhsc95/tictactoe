mod utils;
#[cfg(test)]
use tictactoe::bgame::BinaryGame;
//use tictactoe::strategy::bbest::BinaryBestStrategy;
use tictactoe::bstrategy::bmedium::BinaryMediumStrategy;
use tictactoe::bstrategy::brandom::BinaryRandomStrategy;
use tictactoe::bui::btui::BinaryTerminalUserInterface;

// The idea is just to try the strategy a bunch of times (1000) to see if it doesn't panic.

#[test]
fn brandom_vs_brandom_simulation() {
    for _ in 0..1000000 {
        let ui = BinaryTerminalUserInterface::default();
        let mut tic_tac_toe = BinaryGame::new(ui);
        let random_1 = BinaryRandomStrategy::default();
        let random_2 = BinaryRandomStrategy::default();
        utils::play_by_itself_binary(&mut tic_tac_toe, &random_1, &random_2);
    }
}

// #[test]
// fn bbest_vs_brandom_simulation() {
//     for _ in 0..1000 {
//         let ui = BinaryTerminalUserInterface::default();
//         let mut tic_tac_toe = BinaryGame::new(ui);
//         let best = BinaryBestStrategy::default();
//         let random = BinaryRandomStrategy::default();
//         utils::play_by_itself_binary(&mut tic_tac_toe, &best, &random);
//     }
// }
// #[test]
// fn brandom_vs_bbest_simulation() {
//     for _ in 0..1000 {
//         let ui = BinaryTerminalUserInterface::default();
//         let mut tic_tac_toe = BinaryGame::new(ui);
//         let random = BinaryRandomStrategy::default();
//         let best = BinaryBestStrategy::default();
//         utils::play_by_itself_binary(&mut tic_tac_toe, &random, &best);
//     }
// }

#[test]
fn bmedium_vs_brandom_simulation() {
    for _ in 0..1000 {
        let ui = BinaryTerminalUserInterface::default();
        let mut tic_tac_toe = BinaryGame::new(ui);
        let medium = BinaryMediumStrategy::default();
        let random = BinaryRandomStrategy::default();
        utils::play_by_itself_binary(&mut tic_tac_toe, &medium, &random);
    }
}

#[test]
fn brandom_vs_bmedium_simulation() {
    for _ in 0..1000 {
        let ui = BinaryTerminalUserInterface::default();
        let mut tic_tac_toe = BinaryGame::new(ui);
        let medium = BinaryMediumStrategy::default();
        let random = BinaryRandomStrategy::default();
        utils::play_by_itself_binary(&mut tic_tac_toe, &random, &medium);
    }
}
