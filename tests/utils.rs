#[cfg(test)]
// binary
use tictactoe::bcoodinate::ValidBinaryCoordinate;
use tictactoe::bgame::BinaryGame;
use tictactoe::bstrategy::BinaryStrategy;
use tictactoe::bui::BinaryUserInterface;
// regular
use tictactoe::game::Game;
use tictactoe::player::Player;
use tictactoe::strategy::Strategy;
use tictactoe::ui::UserInterface;

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

pub fn play_by_itself_binary<T: BinaryUserInterface>(
    game: &mut BinaryGame<T>,
    strategy_1: &dyn BinaryStrategy,
    strategy_2: &dyn BinaryStrategy,
) {
    game.bui.display_board(game.board);
    let mut current_player = true;
    while !game.won && game.turn < 9 {
        let c: ValidBinaryCoordinate;
        if current_player {
            c = strategy_1.get_input(current_player, game.board);
        } else {
            c = strategy_2.get_input(current_player, game.board);
        }
        game.make_a_move(c);
        current_player = !current_player;
    }
    if game.won {
        game.bui.display_winner(!current_player);
    } else {
        game.bui.display_draw();
    }
}
