use crate::bcoodinate::ValidBinaryCoordinate;
use crate::bstrategy::BinaryStrategy;
use crate::bui::BinaryUserInterface;

pub struct BinaryGame<T: BinaryUserInterface> {
    pub board: u32,
    pub bui: T,
    pub won: bool,
    pub turn: u8,
}

impl<T: BinaryUserInterface> BinaryGame<T> {
    pub fn new(bui: T) -> Self {
        BinaryGame {
            board: 0,
            bui,
            won: false,
            turn: 0,
        }
    }

    /// Play in multiplayer mode
    pub fn play_multi(&mut self) {
        self.bui.display_board(self.board);
        let mut current_player = true;
        while !self.won && self.turn < 9 {
            match self.bui.get_input(current_player, self.board) {
                Ok(c) => {
                    self.make_a_move(c);
                    current_player = !current_player;
                }
                Err(e) => self.bui.display_error(e),
            }
        }
        if self.won {
            self.bui.display_winner(!current_player);
        } else {
            self.bui.display_draw();
        }
    }

    // Play in single player mode, where the user is the second
    pub fn play_single_second(&mut self, strategy: &dyn BinaryStrategy) {
        let mut current_player = true;
        while !self.won && self.turn < 9 {
            if current_player {
                let c = strategy.get_input(current_player, self.board);
                self.make_a_move(c);
                current_player = !current_player;
            } else {
                match self.bui.get_input(current_player, self.board) {
                    Ok(c) => {
                        self.make_a_move(c);
                        current_player = !current_player;
                    }
                    Err(e) => self.bui.display_error(e),
                }
            }
        }
        if self.won {
            self.bui.display_winner(!current_player);
        } else {
            self.bui.display_draw();
        }
    }

    // Play in single player mode, where the user is the first
    pub fn play_single_first(&mut self, strategy: &dyn BinaryStrategy) {
        self.bui.display_board(self.board);
        let mut current_player = true;
        while !self.won && self.turn < 9 {
            if current_player {
                match self.bui.get_input(current_player, self.board) {
                    Ok(c) => {
                        self.make_a_move(c);
                        current_player = !current_player;
                    }
                    Err(e) => self.bui.display_error(e),
                }
            } else {
                let c = strategy.get_input(current_player, self.board);
                self.make_a_move(c);
                current_player = !current_player;
            }
        }
        if self.won {
            self.bui.display_winner(!current_player);
        } else {
            self.bui.display_draw();
        }
    }

    fn check_win_x(&self) -> bool {
        (self.board & 0b000000000000101010 == 0b000000000000101010)
            || (self.board & 0b000000101010000000 == 0b000000101010000000)
            || (self.board & 0b101010000000000000 == 0b101010000000000000)
            || (self.board & 0b100000100000100000 == 0b100000100000100000)
            || (self.board & 0b001000001000001000 == 0b001000001000001000)
            || (self.board & 0b000010000010000010 == 0b000010000010000010)
            || (self.board & 0b100000001000000010 == 0b100000001000000010)
            || (self.board & 0b000010001000100000 == 0b000010001000100000)
    }

    fn check_win_o(&self) -> bool {
        (self.board & 0b000000000000010101 == 0b000000000000010101)
            || (self.board & 0b000000010101000000 == 0b000000010101000000)
            || (self.board & 0b010101000000000000 == 0b010101000000000000)
            || (self.board & 0b010000010000010000 == 0b010000010000010000)
            || (self.board & 0b000100000100000100 == 0b000100000100000100)
            || (self.board & 0b000001000001000001 == 0b000001000001000001)
            || (self.board & 0b010000000100000001 == 0b010000000100000001)
            || (self.board & 0b000001000100010000 == 0b000001000100010000)
    }

    fn check_win_conditions(&self) -> bool {
        self.check_win_x() || self.check_win_o()
    }

    pub fn make_a_move(&mut self, binary_coordinate: ValidBinaryCoordinate) {
        self.board += 1u32.wrapping_shl(
            ((binary_coordinate.position as u32) << 1) + binary_coordinate.player as u32,
        );
        self.bui.display_board(self.board);
        self.turn += 1;
        self.won = self.check_win_conditions();
    }

    pub fn get_current_player(&self) -> bool {
        if self.turn % 2 == 0 {
            false
        } else {
            true
        }
    }
}
