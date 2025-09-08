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

// bitboard courtesy of https://nereuxofficial.github.io/posts/bitboard-rust/
// note in deriving: https://doc.rust-lang.org/rust-by-example/trait/derive.html
#[derive(PartialEq, Eq, PartialOrd, Clone, Copy, Debug, Default, Hash)]
pub struct BitBoard(pub u64);

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Position{
    //>>>>  NOTEEEE remove pub after testing
    /// Board for each side
    pub bb_sides: [BitBoard; 2],
    // BitBoards for all pieces and each side
    pub bb_pieces: [[BitBoard; 6]; 2],
}

pub struct Sides;
impl Sides {
    pub const WHITE: usize = 0;
    pub const BLACK: usize = 1;
}

pub struct Pieces;
impl Pieces{
    pub const PAWN: usize = 0;
    pub const KNIGHT: usize = 1;
    pub const BISHOP: usize = 2;
    pub const ROOK: usize = 3;
    pub const QUEEN: usize = 4;
    pub const KING: usize = 5;
}

// example: let white_queens: BitBoard = position.bb_pieces[Sides::WHITE][Pieces::QUEEN]; 

// chess board stored as 2d array (? enum or bitboard) ; A->H, 1->8. returns 0 if no piece, returns associated string if piece (i.e. Q = queen)
// string expensive (boo), use enum: i.e. Piece::Queen(Color::White)
// how to keep track white/black?

// error handling: Results, panics, or player-facing messages.
// debugging with https://www.chessprogramming.org/Perft

// special: castling, en passant, promotion

/* int turn = 0;
int currentPlayer: 0; // 0 = white, 1 = black */

// note!!! A1 = 0, H8 = 63 (coord -> bit-index oc vice verse converter behövs)
// >>>> start: player inputs location of piece player wants to move

// nte: snake_case for functions.

/* bitmask functionality:
piece bitboard: 0b0001000000001
mask for D4:    0b0001000000000
AND result:     0b0001000000000 =/= 0 -> square has a piece */

pub fn get_piece_at(position: &Position, square: u8) -> Option<Piece> {
    let mask = 1u64 << square; // "<<" left shift. mask with only bit for target square set, i.e. square = 3 -> mask = 0b1000, 3 bit shifts
    println!("mask: {:064b}", mask);

    for side in [Sides::WHITE, Sides::BLACK] {
        for piece_type in 0..6 {
            println!("position in board: {:064b}", position.bb_pieces[side][piece_type].0);
            if (position.bb_pieces[side][piece_type].0 & mask) != 0 {
                let color = if side == Sides::WHITE {  
                    Color::White
                }else {
                    Color::Black
                };

                let piece = match piece_type{ // order is important!!
                    Pieces::PAWN => Piece::Pawn(color),
                    Pieces::KNIGHT => Piece::Knight(color),
                    Pieces::BISHOP => Piece::Bishop(color),
                    Pieces::ROOK => Piece::Rook(color),
                    Pieces::QUEEN => Piece::Queen(color),
                    Pieces::KING => Piece::King(color),
                    _ => unreachable!(),
                };

                return Some(piece);
            }
        }
    }
    None // since Option
}

/* pub fn playerTracker(){
    /* 
    if (turn % 2 == 0) { // even
        currentPlayer = black; (notated as 1?)
    }else { // uneven -- game starts at 1, i.e. uneven
        current player = white; ()
    } */
}

pub fn turnTracker() {
    // turn++;
}

prv fn checkValidMoves(piece){
    /*
    generate all valid moves first and THEN filter
    check ruleset for type, i.e. if Q: can move all spaces "up" "down" "right" "left" if 0, if string can go no further etc.
    different fn for each type? 
    return valid moves (as array? how to handle a range, i.e. A1-A5?)
     */
}

fn isValidMove {
    /*
    uses array from checkValidMoves (?) and sees if move inputted (i.e. A4) corresponds to one of these. 
    if true: makeMove()
    if false: "not a valid move. try again."
     */
}

fn makeMove {
    /*
    turnTracker();
    mutable access to board
    register the move into the board array (previous position -> 0; new position is filled with type i.e. 0 -> Q)
    then, check for checkmate (and other states?)    
    */
}

fn isChecked {
    /*
    check board 
    states for no check/checked/checkmate
     */
} */