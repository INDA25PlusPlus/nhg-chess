enum Piece {
    Pawn(Color),
    Rook(Color),
    Knight(Color),
    Bishop(Color),
    Queen(Color),
    King(Color),
}
enum Color {
    White,
    Black,
}

// chess board stored as 2d array (? enum or bitboard) ; A->H, 1->8. returns 0 if no piece, returns associated string if piece (i.e. Q = queen)
// string expensive (boo), use enum: i.e. Piece::Queen(Color::White)
// how to keep track white/black?

// error handling: Results, panics, or player-facing messages.
// debugging with https://www.chessprogramming.org/Perft

// special: castling, en passant, promotion

int turn = 0;
int currentPlayer: 0; // 0 = white, 1 = black

// >>>> start: player inputs location of piece player wants to move
pub fn hasPiece(coordinate: &str) -> String {
    // playerTracker()
    // check the two arrays

    // if piece -> check piece ownership vs currentPlayer. if no -> failstate

    /* if String:
        piece = string 
        checkValidMoves(piece);
    if 0: return "coordinate has no piece. please select another coordinate."
    if other: failstate */

    // return true / false
    // if true: checkValidMoves
}

pub fn playerTracker(){
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
}