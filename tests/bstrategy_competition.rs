#[cfg(test)]
use tictactoe::bgame::BinaryGame;
use tictactoe::bstrategy::bbest::BinaryBestStrategy;
use tictactoe::bstrategy::bmedium::BinaryMediumStrategy;
use tictactoe::bstrategy::brandom::BinaryRandomStrategy;
use tictactoe::bui::btui::BinaryTerminalUserInterface;
mod utils;

#[test]
fn bbest_vs_brandom_bbest_wins_or_draw() {
    for _ in 0..1000 {
        let ui = BinaryTerminalUserInterface::default();
        let mut tic_tac_toe = BinaryGame::new(ui);
        let best = BinaryBestStrategy::default();
        let random = BinaryRandomStrategy::default();
        utils::play_by_itself_binary(&mut tic_tac_toe, &best, &random);
        let winner = tic_tac_toe.get_current_player();
        assert!(winner || tic_tac_toe.turn == 9);
    }
}

#[test]
fn brandom_vs_bbest_bbest_does_not_lose() {
    for _ in 0..1000 {
        let ui = BinaryTerminalUserInterface::default();
        let mut tic_tac_toe = BinaryGame::new(ui);
        let random = BinaryRandomStrategy::default();
        let best = BinaryBestStrategy::default();
        utils::play_by_itself_binary(&mut tic_tac_toe, &random, &best);
        let winner = tic_tac_toe.get_current_player();

        assert!(!winner || tic_tac_toe.turn == 9);
    }
}

#[test]
fn bbest_vs_bbest_always_draw() {
    for _ in 0..1000 {
        let ui = BinaryTerminalUserInterface::default();
        let mut tic_tac_toe = BinaryGame::new(ui);
        let best_1 = BinaryBestStrategy::default();
        let best_2 = BinaryBestStrategy::default();
        utils::play_by_itself_binary(&mut tic_tac_toe, &best_1, &best_2);
        let winner = tic_tac_toe.get_current_player();

        assert!(!winner || tic_tac_toe.turn == 9);
    }
}

#[test]
fn bbest_vs_bmedium_bbest_wins_or_draw() {
    for _ in 0..1000 {
        let ui = BinaryTerminalUserInterface::default();
        let mut tic_tac_toe = BinaryGame::new(ui);
        let best = BinaryBestStrategy::default();
        let medium = BinaryMediumStrategy::default();
        utils::play_by_itself_binary(&mut tic_tac_toe, &best, &medium);
        let winner = tic_tac_toe.get_current_player();

        assert!(winner || tic_tac_toe.turn == 9);
    }
}

#[test]
fn bmedium_vs_bbest_bbest_wins_or_draw() {
    for _ in 0..1000 {
        let ui = BinaryTerminalUserInterface::default();
        let mut tic_tac_toe = BinaryGame::new(ui);
        let medium = BinaryMediumStrategy::default();
        let best = BinaryBestStrategy::default();
        utils::play_by_itself_binary(&mut tic_tac_toe, &medium, &best);
        let winner = tic_tac_toe.get_current_player();

        assert!(!winner || tic_tac_toe.turn == 9);
    }
}
