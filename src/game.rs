use crate::piece::Color;
use crate::position::Position;

pub struct Game {
    pub position: Position,
    pub turn: u32,
}

impl Game {
    // constructor 
    pub fn new(position: Position) -> Self {
        Game { position, turn: 1 }
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
}