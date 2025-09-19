use crate::piece::{Piece, Color};
use crate::position::{Position, get_piece_at};

/// Represents the current state of a game.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameResult {
    Ongoing,
    /// One player has been checkmated. Stores the color of the losing side.
    Checkmate(Color),  
    Stalemate,
}

/// Stores the game state, including the board position, turn counter,
/// currently selected piece, and result.
pub struct Game {
    /// Position of all pieces currently on the board.
    pub position: Position,
    /// Current turn number (1 = White’s first move, 2 = Black’s, etc.).
    pub turn: u32,
    /// An optional pair of the currently selected piece and its square index.
    pub selected: Option<(Piece, u8)>,
    pub result: GameResult,
}

impl Game {
    // constructor 
    pub fn new(position: Position) -> Self {
        Game { position, turn: 1, selected: None, result: GameResult::Ongoing, }
    }
    
    /// Increments the turn counter by one.
    ///
    /// Use this after a move is completed.
    pub fn turn_tracker(&mut self) {
        self.turn += 1;
    }

    /// Returns which player's turn it is based on the turn counter.
    ///
    /// Odd turns = White, even turns = Black.
    pub fn player_tracker(&self) -> Color {
        if self.turn % 2 == 1 {
            Color::White
        } else{
            Color::Black
        }
    }

    /// Returns `true` if the game is over (not ongoing).
    pub fn is_over(&self) -> bool {
        self.result != GameResult::Ongoing
    }

    /// Checks if the given piece belongs to the current player.
    ///
    /// # Arguments
    /// * `piece` - The piece to validate.
    ///
    /// # Returns
    /// * `Ok(piece)` if the piece matches the current player's color.
    /// * `Err(&str)` if the piece belongs to the opponent.
    pub fn color_check(&self, piece: Piece) -> Result<Piece, &'static str> {
        let current_color = self.player_tracker();

        match piece {
            Piece::Pawn(c)
            | Piece::Rook(c)
            | Piece::Knight(c)
            | Piece::Bishop(c)
            | Piece::Queen(c)
            | Piece::King(c) => {
                if c == current_color {
                    Ok(piece)
                } else {
                    Err("That piece does not belong to you.") //redundant?
                }
            }
        }
    }

    /// Selects a piece on the given square, if it belongs to the current player.
    ///
    /// # Arguments
    /// * `square` - The board index (in bits) of the piece to select.
    ///
    /// # Returns
    /// * `Ok(piece)` if a valid piece was found and selected.
    /// * `Err(&str)` if the square is empty or contains an opponent's piece.
    pub fn select_piece(&mut self, square: u8) -> Result<Piece, &'static str> {
        match get_piece_at(&self.position, square) {
            Some(piece) => {
                if piece.color() != self.player_tracker() {
                    return Err("You can only select your own pieces");
                }
                self.selected = Some((piece, square));
                Ok(piece)
            }
            None => Err("No piece here"),
        }
    }
}