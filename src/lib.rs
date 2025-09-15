// modules
pub mod piece;
pub mod bitboard;
pub mod position;
pub mod game;
pub mod moves;

// re-exports for easier access from outside
pub use piece::{Piece, Color};
pub use bitboard::BitBoard;
pub use position::{Position, Sides, Pieces, get_piece_at};
pub use game::Game; 
pub use moves::valid_moves;

/* --- notes / future functions ---

// error handling: Results, panics, or player-facing messages.
// debugging with https://www.chessprogramming.org/Perft

// special: castling, en passant, promotion

// note!!! A1 = 0, H8 = 63 (coord -> bit-index oc vice verse converter behövs)
// >>>> start: player inputs location of piece player wants to move

// nte: snake_case for functions.

pub fn playerTracker() { ... }
pub fn turnTracker() { ... }
prv fn checkValidMoves(piece) { ... }
fn isValidMove { ... }
fn makeMove { ... }
fn isChecked { ... }

*/