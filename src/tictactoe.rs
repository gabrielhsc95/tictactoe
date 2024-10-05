use crate::board::Board;
use crate::coordinates::Coordinates;
use crate::player::Player;
use crate::random_strategy::RandomStrategy;
use crate::terminal::Terminal;

fn are_all_same(a: &Option<Player>, b: &Option<Player>, c: &Option<Player>) -> bool {
    match (a, b, c) {
        (Some(x), Some(y), Some(z)) => x == y && y == z,
        _ => false,
    }
}

fn check_win_conditions(board: &Board) -> bool {
    let winning_conditions: [(Option<Player>, Option<Player>, Option<Player>); 8] = [
        (board.matrix[0][0], board.matrix[0][1], board.matrix[0][2]),
        (board.matrix[1][0], board.matrix[1][1], board.matrix[1][2]),
        (board.matrix[2][0], board.matrix[2][1], board.matrix[2][2]),
        (board.matrix[0][0], board.matrix[1][0], board.matrix[2][0]),
        (board.matrix[0][1], board.matrix[1][1], board.matrix[2][1]),
        (board.matrix[0][2], board.matrix[1][2], board.matrix[2][2]),
        (board.matrix[0][0], board.matrix[1][1], board.matrix[2][2]),
        (board.matrix[2][0], board.matrix[1][1], board.matrix[0][2]),
    ];
    for (a, b, c) in winning_conditions {
        let win: bool = are_all_same(&a, &b, &c);
        if win {
            return win;
        }
    }
    false
}
pub struct TicTacToe {
    board: Board,
    ui: Terminal,
    won: bool,
    turn: u8,
}

impl TicTacToe {
    fn get_current_player(&self) -> Player {
        if self.turn % 2 == 0 {
            Player::X
        } else {
            Player::O
        }
    }

    pub fn play_multi(&mut self) {
        while !self.won && self.turn < 9 {
            let current_player: Player = self.get_current_player();
            let c: Coordinates = self.ui.get_input(&current_player, &self.board);
            self.board.update(&c, current_player);
            self.ui.display_board(&self.board);
            self.turn += 1;
            self.won = check_win_conditions(&self.board);
        }
        if self.won {
            self.turn -= 1;
            let current_player: Player = self.get_current_player();
            self.ui.display_winner(&current_player);
        } else {
            self.ui.display_draw();
        }
    }

    pub fn play_single(&mut self, strategy: &RandomStrategy) {
        while !self.won && self.turn < 9 {
            let current_player: Player = self.get_current_player();
            let c: Coordinates;
            if current_player == Player::X {
                c = strategy.get_input(&self.board);
            } else if current_player == Player::O {
                c = self.ui.get_input(&current_player, &self.board);
            } else {
                panic!("It should not end up here!");
            }
            self.board.update(&c, current_player);
            self.ui.display_board(&self.board);
            self.turn += 1;
            self.won = check_win_conditions(&self.board);
        }
        if self.won {
            self.turn -= 1;
            let current_player: Player = self.get_current_player();
            self.ui.display_winner(&current_player);
        } else {
            self.ui.display_draw();
        }
    }

    pub fn new(ui: Terminal) -> Self {
        TicTacToe {
            board: Board::new(),
            ui,
            won: false,
            turn: 0,
        }
    }
}
