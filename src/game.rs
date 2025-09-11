use crate::piece::{Piece, Color};
use crate::position::{Position, get_piece_at};

pub struct Game {
    pub position: Position,
    pub turn: u32,
    pub selected: Option<(Piece, usize)>
}

impl Game {
    // constructor 
    pub fn new(position: Position) -> Self {
        Game { position, turn: 1, selected: None }
    }
    
    // "mut self" -> method takes ownership of the instance and allows it to be mutated within the method.  (borrow mut)
    pub fn turn_tracker(&mut self) {
        self.turn += 1;
    }

    // &self means it checks current state without mutation (borrow immutably)
    pub fn player_tracker(&self) -> Color {
        if self.turn % 2 == 1 {
            Color::White
        } else{
            Color::Black
        }
    }

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
                    Err("that piece does not belong to you poopy boy >:I")
                }
            }
        }
    }

    // Result är "return" https://doc.rust-lang.org/std/result/
    pub fn select_piece(&mut self, square: u8) -> Result<Piece, &'static str> {
        match get_piece_at(&self.position, square) {
            Some(piece) => {
                if piece.color() as Color != self.player_tracker() {
                    return Err("You can only select your own pieces");
                }
                // casting?? ?!?!?! means failure.... feels like this will become problematic...
                self.selected = Some((piece, square as usize));
                println!("selected var: {:?}", self.selected);
                Ok(piece)
            }
            None => Err("No piece here"),
        }
    }
}