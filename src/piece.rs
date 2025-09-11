#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Piece {
    Pawn(Color),
    Rook(Color),
    Knight(Color),
    Bishop(Color),
    Queen(Color),
    King(Color),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Color {
    White,
    Black,
}

impl Piece {
    pub fn color(&self) -> Color {
        match self {
            Piece::Pawn(c)
            | Piece::Knight(c)
            | Piece::Bishop(c)
            | Piece::Rook(c)
            | Piece::Queen(c)
            | Piece::King(c) => *c,
        }
    }
}

