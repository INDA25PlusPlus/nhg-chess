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
/// "Converts" Piece to Color by matching it with a color. Returns Color.
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


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CastlingRights {
    pub white_king_moved: bool,
    pub white_kingside_rook_moved: bool,
    pub white_queenside_rook_moved: bool,
    pub black_king_moved: bool,
    pub black_kingside_rook_moved: bool,
    pub black_queenside_rook_moved: bool,
}
impl CastlingRights {
    pub fn new() -> Self {
        Self {
            white_king_moved: false,
            white_kingside_rook_moved: false,
            white_queenside_rook_moved: false,
            black_king_moved: false,
            black_kingside_rook_moved: false,
            black_queenside_rook_moved: false,
        }
    }
}
