use crate::bcoodinate::ValidBinaryCoordinate;
use crate::bstrategy::bstrategy_utils;
use crate::bstrategy::BinaryStrategy;
use crate::butils;
use crate::error::Result;
use rand::prelude::*;

pub struct BinaryBestStrategy {}

impl BinaryStrategy for BinaryBestStrategy {
    fn get_move(&self, player: bool, board: u32) -> Result<ValidBinaryCoordinate> {
        let empty_elements = butils::get_empty_elements(board);
        let mut rng: ThreadRng = thread_rng();
        if player {
            self.offensive(empty_elements, &mut rng, board)
        } else {
            self.defensive(empty_elements, &mut rng, board)
        }
    }
}

impl BinaryBestStrategy {
    pub fn new() -> Self {
        BinaryBestStrategy {}
    }

    fn defensive(
        &self,
        empty_elements: Vec<u8>,
        rng: &mut ThreadRng,
        board: u32,
    ) -> Result<ValidBinaryCoordinate> {
        // Deffensive is o (false)
        let player = false;
        if empty_elements.len() == 8 {
            // first play
            // try to play middle
            if butils::identify_position_state(4, board) == ' ' {
                ValidBinaryCoordinate::new(4, player, board)
            } else {
                // random if not random corner
                let empty_corners = butils::get_empty_corners(board);
                bstrategy_utils::random_move(empty_corners, player, board, rng)
            }
        } else if empty_elements.len() == 6 {
            // second play
            // not lose
            let not_lose_move = bstrategy_utils::win_move_for_player(board, !player);
            match not_lose_move {
                Some(l) => ValidBinaryCoordinate::new(l.position, player, board),
                None => {
                    let empty_corners = butils::get_empty_corners(board);
                    // if has middle
                    if butils::identify_position_state(4, board) == 'o' {
                        if empty_corners.len() == 2 {
                            let empty_edges = butils::get_empty_edges(board);
                            bstrategy_utils::random_move(empty_edges, player, board, rng)
                        } else {
                            for vc in empty_corners {
                                let [adjacent_1, adjacent_2] = self.get_adjacent_edges(vc);

                                if butils::identify_position_state(adjacent_1, board) != ' '
                                    || butils::identify_position_state(adjacent_2, board) != ' '
                                {
                                    return ValidBinaryCoordinate::new(vc, player, board);
                                }
                            }
                            unreachable!("There should be an available corner at this point!");
                        }
                    } else {
                        bstrategy_utils::random_move(empty_corners, player, board, rng)
                    }
                }
            }
        } else if empty_elements.len() == 4 {
            // third play
            let win_move = bstrategy_utils::win_move_for_player(board, player);
            return match win_move {
                // if possible win
                Some(w) => ValidBinaryCoordinate::from(&w, board),
                // else not lose or random
                None => {
                    let not_lose_move = bstrategy_utils::win_move_for_player(board, !player);
                    match not_lose_move {
                        Some(l) => ValidBinaryCoordinate::new(l.position, player, board),
                        None => bstrategy_utils::random_move(empty_elements, player, board, rng),
                    }
                }
            };
        } else if empty_elements.len() == 2 {
            // forth play
            let win_move = bstrategy_utils::win_move_for_player(board, player);
            return match win_move {
                // if possible win
                Some(w) => ValidBinaryCoordinate::from(&w, board),
                // else not lose or random
                None => {
                    let not_lose_move = bstrategy_utils::win_move_for_player(board, !player);
                    match not_lose_move {
                        Some(l) => ValidBinaryCoordinate::new(l.position, player, board),
                        None => bstrategy_utils::random_move(empty_elements, player, board, rng),
                    }
                }
            };
        } else {
            // last play
            let win_move = bstrategy_utils::win_move_for_player(board, player);
            return match win_move {
                // if possible win
                Some(w) => ValidBinaryCoordinate::from(&w, board),
                // else not lose or random
                None => {
                    let not_lose_move = bstrategy_utils::win_move_for_player(board, !player);
                    match not_lose_move {
                        Some(l) => ValidBinaryCoordinate::new(l.position, player, board),
                        None => bstrategy_utils::random_move(empty_elements, player, board, rng),
                    }
                }
            };
        }
    }

    fn get_first_play(&self, board: u32) -> u8 {
        for p in [0, 2, 6, 8] {
            if butils::identify_position_state(p, board) == 'x' {
                return p;
            }
        }
        unreachable!("The first must have happpened in a corner");
    }

    fn get_opposite_position(&self, position: u8) -> u8 {
        8 - position
    }

    fn get_adjacent_edges(&self, corner: u8) -> [u8; 2] {
        match corner {
            0 => [1, 3],
            2 => [5, 1],
            6 => [3, 7],
            8 => [5, 7],
            _ => unreachable!("It is not corner!"),
        }
    }

    fn get_adjacent_corners(&self, corner: u8) -> [u8; 2] {
        match corner {
            0 => [2, 6],
            2 => [0, 8],
            6 => [0, 8],
            8 => [2, 6],
            _ => unreachable!("It is not corner!"),
        }
    }

    fn get_edge_in_between(&self, corner_1: u8, corner_2: u8) -> u8 {
        match corner_1 {
            0 => match corner_2 {
                2 => 1,
                6 => 3,
                _ => unreachable!("Not a valid set of corner pairs!"),
            },
            2 => match corner_2 {
                0 => 1,
                8 => 5,
                _ => unreachable!("Not a valid set of corner pairs!"),
            },
            6 => match corner_2 {
                0 => 3,
                8 => 7,
                _ => unreachable!("Not a valid set of corner pairs!"),
            },
            8 => match corner_2 {
                2 => 5,
                6 => 7,
                _ => unreachable!("Not a valid set of corner pairs!"),
            },
            _ => unreachable!("It is not corner!"),
        }
    }

    fn offensive(
        &self,
        empty_elements: Vec<u8>,
        rng: &mut ThreadRng,
        board: u32,
    ) -> Result<ValidBinaryCoordinate> {
        // offensive is x (true)
        let player = true;
        if empty_elements.len() == 9 {
            // first play
            // random corner
            let empty_corners = butils::get_empty_corners(board);
            return bstrategy_utils::random_move(empty_corners, player, board, rng);
        } else if empty_elements.len() == 7 {
            // second play
            let first_play_position = self.get_first_play(board);
            if butils::identify_position_state(4, board) == 'o' {
                // second player chose middle
                // opposite corner
                let position = self.get_opposite_position(first_play_position);
                return ValidBinaryCoordinate::new(position, player, board);
            } else {
                // adjacent corner that is not blocked
                for corner in self.get_adjacent_corners(first_play_position) {
                    let adjecent_edge = self.get_edge_in_between(first_play_position, corner);
                    if butils::identify_position_state(corner, board) == ' '
                        && butils::identify_position_state(adjecent_edge, board) == ' '
                    {
                        return ValidBinaryCoordinate::new(corner, player, board);
                    }
                }
                unreachable!("There must be a corner");
            }
        } else if empty_elements.len() == 5 {
            // third play
            let win_move = bstrategy_utils::win_move_for_player(board, player);
            match win_move {
                // if possible win
                Some(w) => ValidBinaryCoordinate::from(&w, board),
                // else not lose or another corner
                None => {
                    let not_lose_move = bstrategy_utils::win_move_for_player(board, !player);
                    match not_lose_move {
                        Some(l) => ValidBinaryCoordinate::new(l.position, player, board),
                        None => {
                            for corner in butils::get_empty_corners(board) {
                                let [adjecent_edge_1, adjecent_edge_2] =
                                    self.get_adjacent_edges(corner);
                                if butils::identify_position_state(adjecent_edge_1, board) == ' '
                                    && butils::identify_position_state(adjecent_edge_2, board)
                                        == ' '
                                {
                                    return ValidBinaryCoordinate::new(corner, player, board);
                                }
                            }
                            unreachable!("There must be a valid corner with no adjacent elements!");
                        }
                    }
                }
            }
        } else if empty_elements.len() == 3 {
            // forth play
            let win_move = bstrategy_utils::win_move_for_player(board, player);
            match win_move {
                // if possible win
                Some(w) => ValidBinaryCoordinate::from(&w, board),
                None => {
                    // else not lose or draw so random
                    let not_lose_move = bstrategy_utils::win_move_for_player(board, !player);
                    match not_lose_move {
                        Some(l) => ValidBinaryCoordinate::new(l.position, player, board),
                        None => bstrategy_utils::random_move(empty_elements, player, board, rng),
                    }
                }
            }
        } else {
            // last play
            // random move
            bstrategy_utils::random_move(empty_elements, player, board, rng)
        }
    }
}

impl Default for BinaryBestStrategy {
    fn default() -> Self {
        Self::new()
    }
}
