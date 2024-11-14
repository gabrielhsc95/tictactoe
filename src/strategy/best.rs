use super::Strategy;
use crate::board::Board;
use crate::coordinate::{Coordinate, SafeCoordinate};
use crate::error::{Error, Result};
use crate::player::Player;
use rand::prelude::*;

pub struct BestStrategy {}

impl Strategy for BestStrategy {
    fn get_move(&self, board: &Board) -> Result<Coordinate> {
        let empty_elements: Vec<SafeCoordinate> = board.get_empties_elements();
        let mut rng: ThreadRng = thread_rng();
        if empty_elements.len() == 9 {
            // first play
            // random corner
            return self.get_one_random_valid_corner(board, &mut rng);
        } else if empty_elements.len() == 7 {
            // second play
            let first_play: SafeCoordinate = self.get_first_play(board);
            if !board.matrix[1][1].is_none() {
                // second player chose middle
                // opposite corner
                return Coordinate::new(
                    self.opposite_value(first_play.0),
                    self.opposite_value(first_play.1),
                    board,
                );
            } else {
                // adjacent corner that is not blocked
                if board.matrix[first_play.0][1].is_none() {
                    // between the first_play and (first_play.0, opposite_value(first_play.1)),
                    // which is (first_play.0, 1) is free
                    return Coordinate::new(first_play.0, self.opposite_value(first_play.1), board);
                } else {
                    // if blocked then the other adjacent is (opposite_value(first_play.0), first_play.1)
                    return Coordinate::new(self.opposite_value(first_play.0), first_play.1, board);
                }
            }
        } else if empty_elements.len() == 5 {
            // third play
            let win_move = self.win_move_for_player(board, Player::X);
            match win_move {
                // if possible win
                Some(w) => Coordinate::from(&w, board),
                // else not lose or another corner
                None => {
                    let not_lose_move = self.win_move_for_player(board, Player::O);
                    match not_lose_move {
                        Some(l) => Coordinate::from(&l, board),
                        None => self.get_one_random_valid_corner(board, &mut rng),
                    }
                }
            }
        } else if empty_elements.len() == 3 {
            // forth play
            let win_move = self.win_move_for_player(board, Player::X);
            match win_move {
                // if possible win
                Some(w) => Coordinate::from(&w, board),
                None => {
                    // else TODO: **not lose** or draw so random
                    let not_lose_move = self.win_move_for_player(board, Player::O);
                    match not_lose_move {
                        Some(l) => Coordinate::from(&l, board),
                        None => {
                            let m: SafeCoordinate = self.random_move(empty_elements, &mut rng);
                            return Coordinate::from(&m, board);
                        }
                    }
                }
            }
        } else {
            // last play
            // random
            let m: SafeCoordinate = self.random_move(empty_elements, &mut rng);
            return Coordinate::from(&m, board);
        }
    }
}

impl BestStrategy {
    pub fn new() -> Self {
        BestStrategy {}
    }

    fn get_valid_corners(&self, board: &Board) -> Vec<Coordinate> {
        let corners: Vec<SafeCoordinate> = board.get_corners();
        let mut valid_corners: Vec<Coordinate> = Vec::new();
        for c in corners {
            match Coordinate::from(&c, board) {
                Ok(coord) => valid_corners.push(coord),
                Err(_) => {}
            }
        }
        valid_corners
    }

    fn get_one_random_valid_corner(
        &self,
        board: &Board,
        rng: &mut ThreadRng,
    ) -> Result<Coordinate> {
        let valid_corners: Vec<Coordinate> = self.get_valid_corners(board);
        let coordinate: Option<&Coordinate> = valid_corners.choose(rng);
        match coordinate {
            Some(c) => Ok(c.clone()),
            None => Err(Error::StrategyInvalidMove(String::from(
                "There must be corner available!",
            ))),
        }
    }

    fn get_first_play(&self, board: &Board) -> SafeCoordinate {
        // don't use this function if it is not the second play of the CPU
        let corners: Vec<SafeCoordinate> = board.get_corners();
        self.find_x(corners, board)
    }

    fn find_x(&self, coordinates: Vec<SafeCoordinate>, board: &Board) -> SafeCoordinate {
        for c in coordinates {
            let on_the_board = board.matrix[c.1][c.0];
            match on_the_board {
                Some(p) => {
                    if p == Player::X {
                        return c;
                    }
                }
                None => {}
            }
        }
        unreachable!("It should not be here!");
    }

    fn opposite_value(&self, index: usize) -> usize {
        match index {
            0 => 2,
            2 => 0,
            _ => index,
        }
    }

    fn win_move_for_player(&self, board: &Board, player: Player) -> Option<SafeCoordinate> {
        let winning_conditions = board.get_winning_conditions();
        for ((c0, c1, c2), (v0, v1, v2)) in winning_conditions {
            let num: u8 = self.count_player((v0, v1, v2), player);
            if num == 2 {
                if v0.is_none() {
                    return Some(c0);
                } else if v1.is_none() {
                    return Some(c1);
                } else if v2.is_none() {
                    return Some(c2);
                }
            }
        }
        None
    }

    fn count_player(
        &self,
        condition: (Option<Player>, Option<Player>, Option<Player>),
        player: Player,
    ) -> u8 {
        let mut num: u8 = 0;
        for c in [condition.0, condition.1, condition.2] {
            if Some(player) == c {
                num += 1;
            }
        }
        num
    }

    fn random_move(
        &self,
        empty_elements: Vec<SafeCoordinate>,
        rng: &mut ThreadRng,
    ) -> SafeCoordinate {
        let coordinate = empty_elements
            .choose(rng)
            .expect("There should be one last valid coordinate!");
        return coordinate.clone();
    }
}
