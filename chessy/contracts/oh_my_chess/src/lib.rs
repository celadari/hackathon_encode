#![cfg_attr(not(feature = "std"), no_std, no_main)]

extern crate alloc;
use ink;
use pink_extension as pink;

use scale::{Encode, Decode};

#[pink::contract(env=PinkEnvironment)]
mod oh_my_chess {
    use super::*;
    use mongodb::{Client, options::ClientOptions};

    use alloc::collections::BTreeMap;
    use pink::chain_extension::pink_extension_instance as ext;
    use pink::PinkEnvironment;
    pub type GameId = u64;


    #[ink(storage)]
    pub struct OhMyChess {
        admin: AccountId,
        url: String,
    }

    impl OhMyChess {

        #[ink(constructor)]
        pub fn default() -> Self {
            Self {
                admin: Self::env().caller(),
                url: String::default()
            }
        }

        #[ink(message)]
        pub fn get_url(&self) -> String {
            self.url.clone()
        }

        #[ink(message)]
        pub fn set_url(mut& self, url: String) -> String {
            self.url = url.clone();
        }

        pub fn check_move_validity(&self, chess_move: &ChessMove, player_to_move: &Player, game_state: &GameState) -> bool {
            // Check if the move is within the board boundaries
            if chess_move.from.0 < 0 || chess_move.from.0 > 7 || chess_move.from.1 < 0 || chess_move.from.1 > 7 || chess_move.to.0 < 0 || chess_move.to.0 > 7 || chess_move.to.1 < 0 || chess_move.to.1 > 7 {
                return false;
            }

            let (piece, player) = match game_state.board[chess_move.from.0 as usize][chess_move.from.1 as usize] {
                Some(ref tuple) => tuple,
                None => return false, // No piece at source
            };

            if player_to_move != player {
                return false;
            }

            let delta_row = (chess_move.from.0 as i8 - chess_move.to.0 as i8).abs();
            let delta_col = (chess_move.from.1 as i8 - chess_move.to.1 as i8).abs();

            match piece {
                Piece::Pawn => true,
                Piece::Knight => true,
                Piece::Bishop => true,
                Piece::Rook => true,
                Piece::Queen => true,
                Piece::King => true,
            }

            true
        }

        fn check_move_validity_king(&self, chess_move: &ChessMove, player_to_move: &Player, game_state: &GameState) -> bool {
            // Calculate the difference in the move for both axes
            let delta_row = (chess_move.from.0 as i8 - chess_move.to.0 as i8).abs();
            let delta_col = (chess_move.from.1 as i8 - chess_move.to.1 as i8).abs();

            if (delta_row <= 1 && delta_col <= 1) { true } else { false }
        }

        fn check_move_validity_queen(&self, chess_move: &ChessMove, player_to_move: &Player, game_state: &GameState) -> bool {
            // Queen can move horizontally, vertically, or diagonally
            let from = chess_move.from;
            let to = chess_move.to;
            let is_horizontal = from.0 == to.0;
            let is_vertical = from.1 == to.1;
            let is_diagonal = (from.0 as i32 - to.0 as i32).abs() == (from.1 as i32 - to.1 as i32).abs();

            if is_horizontal {
                self.is_path_clear(&game_state.board, from, to, &Direction::Horizontal)
            } else if is_vertical {
                self.is_path_clear(&game_state.board, from, to, &Direction::Vertical)
            } else if is_diagonal {
                self.is_path_clear(&game_state.board, from, to, &Direction::Diagonal)
            } else {
                false // Not a valid queen move
            }
        }

        fn is_path_clear(board: &[[Option<(Piece, Player)>; 8]; 8], from: (usize, usize), to: (usize, usize), direction: &Direction) -> bool {
            let (dx, dy) = (to.0 as i32 - from.0 as i32, to.1 as i32 - from.1 as i32);
            let step_x = dx.signum() as usize;
            let step_y = dy.signum() as usize;

            // Check if movement is according to the piece's moving pattern
            match direction {
                Direction::Horizontal => if dy != 0 { return false; },
                Direction::Vertical => if dx != 0 { return false; },
                Direction::Diagonal => if dx.abs() != dy.abs() { return false; },
            }

            let mut current_x = from.0;
            let mut current_y = from.1;

            while (current_x, current_y) != (to.0, to.1) {
                current_x += step_x;
                current_y += step_y;

                // Avoid checking the destination square for a piece
                if (current_x, current_y) == (to.0, to.1) {
                    break;
                }

                // Check if the path is clear
                if board[current_x][current_y].is_some() {
                    return false;
                }
            }
            true
        }
    }

    #[derive(Encode, Decode, Clone, Debug, PartialEq)]
    pub enum Direction {
        Horizontal,
        Vertical,
        Diagonal,
    }

    pub enum Piece {
        Pawn, Knight, Bishop, Rook, Queen, King
    }

    #[derive(Encode, Decode, Clone, Debug, PartialEq)]
    pub enum Player {
        Black, White
    }

    #[derive(Encode, Decode, Clone, Debug, PartialEq)]
    pub enum GameStatus {
        Ongoing, Checkmate, Stalemate, Draw
    }

    #[derive(Encode, Decode, Clone, Debug)]
    pub struct GameState {
        board: [[Option<(Piece, Player)>; 8]; 8],
        turn: Player,
        players: (AccountId, AccountId),
        status: GameStatus,

    }

    #[derive(Encode, Decode, Clone, Debug, PartialEq)]
    pub struct ChessMove {
        // Define your move structure
        from: (u8, u8),
        to: (u8, u8),
    }

}
