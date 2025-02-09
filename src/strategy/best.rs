//! Best Strategy, it never loses.
use super::Strategy;
use crate::board::Board;
use crate::coordinate::{Coordinate, ValidCoordinate};
use crate::error::Result;
use crate::player::Player;
use crate::strategy::utils;
use rand::prelude::*;

pub struct BestStrategy {}

impl Strategy for BestStrategy {
    fn get_move(&self, board: &Board) -> Result<ValidCoordinate> {
        let empty_elements: Vec<ValidCoordinate> = board.get_empty_elements();
        let mut rng: ThreadRng = thread_rng();

        if empty_elements.len() % 2 == 0 {
            self.defensive(empty_elements, &mut rng, board)
        } else {
            self.offensive(empty_elements, &mut rng, board)
        }
    }
}

impl BestStrategy {
    pub fn new() -> Self {
        BestStrategy {}
    }
    /// Back tracks what was done as a first move
    ///
    /// Do not use this function if it is not the second play of the CPU
    fn get_first_play(&self, board: &Board) -> Coordinate {
        let corners: Vec<Coordinate> = board.get_corners();
        self.find_x(corners, board)
    }

    /// Find the first Player X for given Vec<Coordinate>
    fn find_x(&self, coordinates: Vec<Coordinate>, board: &Board) -> Coordinate {
        for c in coordinates {
            let on_the_board = board.matrix[c.1][c.0];
            if let Some(p) = on_the_board {
                if p == Player::X {
                    return c;
                }
            }
        }
        unreachable!("It should not be here!");
    }

    /// Return the opposite coordinate
    fn opposite_value(&self, index: usize) -> usize {
        match index {
            0 => 2,
            2 => 0,
            _ => index,
        }
    }

    /// Plays offensive move, it starts
    fn offensive(
        &self,
        empty_elements: Vec<ValidCoordinate>,
        rng: &mut ThreadRng,
        board: &Board,
    ) -> Result<ValidCoordinate> {
        if empty_elements.len() == 9 {
            // first play
            // random corner
            let empty_corners = board.get_empty_corners();
            utils::random_move(empty_corners, rng)
        } else if empty_elements.len() == 7 {
            // second play
            let first_play: Coordinate = self.get_first_play(board);
            if board.matrix[1][1].is_some() {
                // second player chose middle
                // opposite corner
                return ValidCoordinate::new(
                    self.opposite_value(first_play.0),
                    self.opposite_value(first_play.1),
                    board,
                );
            } else {
                // adjacent corner that is not blocked
                let opposite_first_play_0 = self.opposite_value(first_play.0);
                let opposite_first_play_1 = self.opposite_value(first_play.1);
                if board.matrix[1][first_play.0].is_none()
                    && board.matrix[opposite_first_play_1][first_play.0].is_none()
                {
                    // between the first_play and (first_play.0, opposite_first_play_1),
                    // which is (first_play.0, 1) is free,
                    // but also (first_play.0, opposite_first_play_1),
                    // so we can play there
                    return ValidCoordinate::new(first_play.0, opposite_first_play_1, board);
                } else {
                    // if blocked then the other adjacent is (opposite_first_play_0, first_play.1),
                    // but we don't need to check that corner
                    return ValidCoordinate::new(opposite_first_play_0, first_play.1, board);
                }
            }
        } else if empty_elements.len() == 5 {
            // third play
            let win_move = utils::win_move_for_player(board, Player::X);
            match win_move {
                // if possible win
                Some(w) => ValidCoordinate::from(&w, board),
                // else not lose or another corner
                None => {
                    let not_lose_move = utils::win_move_for_player(board, Player::O);
                    match not_lose_move {
                        Some(l) => ValidCoordinate::from(&l, board),
                        None => {
                            let empty_corners = board.get_empty_corners();
                            for vc in empty_corners {
                                if board.matrix[1][vc.x()].is_none()
                                    && board.matrix[vc.y()][1].is_none()
                                {
                                    return Ok(vc);
                                }
                            }
                            unreachable!("There must be a valid corner with no adjacent elements!");
                        }
                    }
                }
            }
        } else if empty_elements.len() == 3 {
            // forth play
            let win_move = utils::win_move_for_player(board, Player::X);
            match win_move {
                // if possible win
                Some(w) => ValidCoordinate::from(&w, board),
                None => {
                    // else not lose or draw so random
                    let not_lose_move = utils::win_move_for_player(board, Player::O);
                    match not_lose_move {
                        Some(l) => ValidCoordinate::from(&l, board),
                        None => utils::random_move(empty_elements, rng),
                    }
                }
            }
        } else {
            // last play
            // random move
            utils::random_move(empty_elements, rng)
        }
    }

    /// Play defensive mode, the enemy starts.
    fn defensive(
        &self,
        empty_elements: Vec<ValidCoordinate>,
        rng: &mut ThreadRng,
        board: &Board,
    ) -> Result<ValidCoordinate> {
        if empty_elements.len() == 8 {
            // first play
            // try to play middle
            if board.matrix[1][1].is_none() {
                ValidCoordinate::new(1, 1, board)
            } else {
                // random if not random corner
                let empty_corners = board.get_empty_corners();
                utils::random_move(empty_corners, rng)
            }
        } else if empty_elements.len() == 6 {
            // second play
            // not lose
            let not_lose_move = utils::win_move_for_player(board, Player::X);
            match not_lose_move {
                Some(l) => ValidCoordinate::from(&l, board),
                None => {
                    let has_middle = match board.matrix[1][1] {
                        Some(player) => player == Player::O,
                        None => false,
                    };
                    if has_middle {
                        let empty_corners = board.get_empty_corners();
                        if empty_corners.len() == 2 {
                            let empty_edges = board.get_empty_edges();
                            utils::random_move(empty_edges, rng)
                        } else {
                            for vc in empty_corners {
                                let adjacent_1 = board.matrix[1][vc.x()];
                                let adjacent_2 = board.matrix[vc.y()][1];

                                if adjacent_1.is_some() || adjacent_2.is_some() {
                                    return Ok(vc);
                                }
                            }
                            unreachable!("There should be an available corner at this point!");
                        }
                    } else {
                        let empty_corners = board.get_empty_corners();
                        utils::random_move(empty_corners, rng)
                    }
                }
            }
        } else if empty_elements.len() == 4 {
            // third play
            let win_move = utils::win_move_for_player(board, Player::O);
            return match win_move {
                // if possible win
                Some(w) => ValidCoordinate::from(&w, board),
                // else not lose or random
                None => {
                    let not_lose_move = utils::win_move_for_player(board, Player::X);
                    match not_lose_move {
                        Some(l) => ValidCoordinate::from(&l, board),
                        None => utils::random_move(empty_elements, rng),
                    }
                }
            };
        } else if empty_elements.len() == 2 {
            // forth play
            let win_move = utils::win_move_for_player(board, Player::O);
            return match win_move {
                // if possible win
                Some(w) => ValidCoordinate::from(&w, board),
                // else not lose or random
                None => {
                    let not_lose_move = utils::win_move_for_player(board, Player::X);
                    match not_lose_move {
                        Some(l) => ValidCoordinate::from(&l, board),
                        None => utils::random_move(empty_elements, rng),
                    }
                }
            };
        } else {
            // last play
            let win_move = utils::win_move_for_player(board, Player::O);
            return match win_move {
                // if possible win
                Some(w) => ValidCoordinate::from(&w, board),
                // else not lose or random
                None => {
                    let not_lose_move = utils::win_move_for_player(board, Player::X);
                    match not_lose_move {
                        Some(l) => ValidCoordinate::from(&l, board),
                        None => utils::random_move(empty_elements, rng),
                    }
                }
            };
        }
    }
}

impl Default for BestStrategy {
    fn default() -> Self {
        Self::new()
    }
}
