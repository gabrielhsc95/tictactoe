#[cfg(test)]
use tictactoe::game::Game;
use tictactoe::player::Player;
use tictactoe::strategy::best::BestStrategy;
use tictactoe::strategy::random::RandomStrategy;
use tictactoe::ui::tui::TerminalUserInterface;
#[test]
fn best_vs_random_best_wins_or_draw() {
    let ui = TerminalUserInterface::new();
    let mut tic_tac_toe = Game::new(ui);
    let best_strategy = BestStrategy::new();
    let random_strategy = RandomStrategy::new();
    tic_tac_toe.play_by_itself(&best_strategy, &random_strategy);
    let winner = tic_tac_toe.get_current_player();

    assert!(winner == Player::X || tic_tac_toe.turn == 9);
}
