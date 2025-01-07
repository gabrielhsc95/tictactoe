#[cfg(test)]
use tictactoe::game::Game;
use tictactoe::strategy::best::BestStrategy;
use tictactoe::strategy::medium::MediumStrategy;
use tictactoe::strategy::random::RandomStrategy;
use tictactoe::ui::tui::TerminalUserInterface;

// The idea is just to try the strategy a bunch of times (1000) to see if it doesn't panic.

#[test]
fn random_vs_random_simulation() {
    for _ in 0..1000 {
        let ui = TerminalUserInterface::new();
        let mut tic_tac_toe = Game::new(ui);
        let random_1 = RandomStrategy::new();
        let random_2 = RandomStrategy::new();
        tic_tac_toe.play_by_itself(&random_1, &random_2);
    }
}
#[test]
fn best_vs_random_simulation() {
    for _ in 0..1000 {
        let ui = TerminalUserInterface::new();
        let mut tic_tac_toe = Game::new(ui);
        let best = BestStrategy::new();
        let random = RandomStrategy::new();
        tic_tac_toe.play_by_itself(&best, &random);
    }
}
#[test]
fn random_vs_best_simulation() {
    for _ in 0..1000 {
        let ui = TerminalUserInterface::new();
        let mut tic_tac_toe = Game::new(ui);

        let random = RandomStrategy::new();
        let best = BestStrategy::new();
        tic_tac_toe.play_by_itself(&random, &best);
    }
}

#[test]
fn medium_vs_random_simulation() {
    for _ in 0..1000 {
        let ui = TerminalUserInterface::new();
        let mut tic_tac_toe = Game::new(ui);
        let medium = MediumStrategy::new();
        let random = RandomStrategy::new();
        tic_tac_toe.play_by_itself(&medium, &random);
    }
}

#[test]
fn random_vs_medium_simulation() {
    for _ in 0..1000 {
        let ui = TerminalUserInterface::new();
        let mut tic_tac_toe = Game::new(ui);
        let medium = MediumStrategy::new();
        let random = RandomStrategy::new();
        tic_tac_toe.play_by_itself(&random, &medium);
    }
}
