use crate::board::Board;
use crate::coordinate::ValidCoordinate;
use crate::player::Player;
use crate::strategy::Strategy;
use crate::ui::UserInterface;

fn are_all_same(a: &Option<Player>, b: &Option<Player>, c: &Option<Player>) -> bool {
    match (a, b, c) {
        (Some(x), Some(y), Some(z)) => x == y && y == z,
        _ => false,
    }
}

fn check_win_conditions(board: &Board) -> bool {
    let winning_conditions = board.get_winning_conditions();
    for (a, b, c) in winning_conditions.values() {
        let win: bool = are_all_same(&a, &b, &c);
        if win {
            return win;
        }
    }
    false
}
pub struct Game<T: UserInterface> {
    board: Board,
    ui: T,
    won: bool,
    pub turn: u8,
}

impl<T: UserInterface> Game<T> {
    pub fn get_current_player(&self) -> Player {
        if self.turn % 2 == 0 {
            Player::X
        } else {
            Player::O
        }
    }

    pub fn play_multi(&mut self) {
        while !self.won && self.turn < 9 {
            let current_player: Player = self.get_current_player();
            match self.ui.get_input(&current_player, &self.board) {
                Ok(coord) => {
                    let c: ValidCoordinate = coord;
                    self.make_a_move(&c, current_player);
                }
                Err(e) => self.ui.display_error(e),
            }
        }
        if self.won {
            self.turn -= 1;
            let current_player: Player = self.get_current_player();
            self.ui.display_winner(&current_player);
        } else {
            self.ui.display_draw();
        }
    }

    pub fn play_single(&mut self, strategy: &dyn Strategy) {
        while !self.won && self.turn < 9 {
            let current_player: Player = self.get_current_player();
            match current_player {
                Player::X => {
                    let c = strategy.get_input(&self.board);
                    self.make_a_move(&c, current_player);
                }
                Player::O => match self.ui.get_input(&current_player, &self.board) {
                    Ok(c) => {
                        self.make_a_move(&c, current_player);
                    }
                    Err(e) => self.ui.display_error(e),
                },
            }
        }
        if self.won {
            self.turn -= 1;
            let current_player: Player = self.get_current_player();
            self.ui.display_winner(&current_player);
        } else {
            self.ui.display_draw();
        }
    }

    // for testing
    pub fn play_by_itself(&mut self, strategy_1: &dyn Strategy, strategy_2: &dyn Strategy) {
        while !self.won && self.turn < 9 {
            let current_player: Player = self.get_current_player();
            match current_player {
                Player::X => {
                    let c = strategy_1.get_input(&self.board);
                    self.make_a_move(&c, current_player);
                }
                Player::O => {
                    let c = strategy_2.get_input(&self.board);
                    self.make_a_move(&c, current_player);
                }
            }
        }
        if self.won {
            self.turn -= 1;
            let current_player: Player = self.get_current_player();
            self.ui.display_winner(&current_player);
        } else {
            self.ui.display_draw();
        }
    }

    fn make_a_move(&mut self, coordinate: &ValidCoordinate, player: Player) {
        self.board.update(coordinate, player);
        self.ui.display_board(&self.board);
        self.turn += 1;
        self.won = check_win_conditions(&self.board);
    }

    pub fn new(ui: T) -> Self {
        Game {
            board: Board::new(),
            ui,
            won: false,
            turn: 0,
        }
    }
}
