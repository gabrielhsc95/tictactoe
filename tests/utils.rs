#[cfg(test)]
use tictactoe::game::Game;
use tictactoe::player::Player;
use tictactoe::strategy::Strategy;
use tictactoe::ui::UserInterface;

#[cfg(test)]
pub fn play_by_itself<T: UserInterface>(
    game: &mut Game<T>,
    strategy_1: &dyn Strategy,
    strategy_2: &dyn Strategy,
) {
    while !game.won && game.turn < 9 {
        let current_player: Player = game.get_current_player();
        match current_player {
            Player::X => {
                let c = strategy_1.get_input(&game.board);
                game.make_a_move(&c, current_player);
            }
            Player::O => {
                let c = strategy_2.get_input(&game.board);
                game.make_a_move(&c, current_player);
            }
        }
    }
    if game.won {
        game.turn -= 1;
        let current_player: Player = game.get_current_player();
        game.ui.display_winner(&current_player);
    } else {
        game.ui.display_draw();
    }
}
