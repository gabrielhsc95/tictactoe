#[cfg(test)]
use tictactoe::game::Game;
use tictactoe::player::Player;
use tictactoe::strategy::best::BestStrategy;
use tictactoe::strategy::random::RandomStrategy;
use tictactoe::ui::tui::TerminalUserInterface;
mod utils;

#[test]
fn best_vs_random_best_wins_or_draw() {
    for _ in 0..1000 {
        let ui = TerminalUserInterface::new();
        let mut tic_tac_toe = Game::new(ui);
        let best = BestStrategy::new();
        let random = RandomStrategy::new();
        utils::play_by_itself(&mut tic_tac_toe, &best, &random);
        let winner = tic_tac_toe.get_current_player();

        assert!(winner == Player::X || tic_tac_toe.turn == 9);
    }
}

#[test]
fn random_vs_best_best_does_not_lose() {
    for _ in 0..1000 {
        let ui = TerminalUserInterface::new();
        let mut tic_tac_toe = Game::new(ui);
        let random = RandomStrategy::new();
        let best = BestStrategy::new();
        utils::play_by_itself(&mut tic_tac_toe, &random, &best);
        let winner = tic_tac_toe.get_current_player();

        assert!(winner != Player::X || tic_tac_toe.turn == 9);
    }
}

#[test]
fn best_vs_best_always_draw() {
    for _ in 0..1000 {
        let ui = TerminalUserInterface::new();
        let mut tic_tac_toe = Game::new(ui);
        let best_1 = BestStrategy::new();
        let best_2 = BestStrategy::new();
        utils::play_by_itself(&mut tic_tac_toe, &best_1, &best_2);
        let winner = tic_tac_toe.get_current_player();

        assert!(winner != Player::X || tic_tac_toe.turn == 9);
    }
}

#[test]
fn best_vs_medium_best_wins_or_draw() {
    for _ in 0..1000 {
        let ui = TerminalUserInterface::new();
        let mut tic_tac_toe = Game::new(ui);
        let best = BestStrategy::new();
        let medium = BestStrategy::new();
        utils::play_by_itself(&mut tic_tac_toe, &best, &medium);
        let winner = tic_tac_toe.get_current_player();

        assert!(winner == Player::X || tic_tac_toe.turn == 9);
    }
}

#[test]
fn medium_vs_best_best_wins_or_draw() {
    for _ in 0..1000 {
        let ui = TerminalUserInterface::new();
        let mut tic_tac_toe = Game::new(ui);
        let medium = BestStrategy::new();
        let best = BestStrategy::new();
        utils::play_by_itself(&mut tic_tac_toe, &medium, &best);
        let winner = tic_tac_toe.get_current_player();

        assert!(winner == Player::O || tic_tac_toe.turn == 9);
    }
}
