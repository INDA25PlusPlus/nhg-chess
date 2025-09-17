// name of library is defined in cargo.toml
use chess::*;

fn main() {
    let mut position = Position {
        bb_sides: [BitBoard(0), BitBoard(1)], //initalize bitboard
        bb_pieces: [[BitBoard(0); 6]; 2], 
    };

    // white pawn placed on 0
    // position.bb_pieces[Sides::WHITE][Pieces::PAWN] = BitBoard(1 << 0); 
    // black KNIGHT placed on 3
    let square: u8 = 27;
    position.bb_pieces[Sides::WHITE][Pieces::PAWN] = BitBoard(1 << square);  // fyi this overwrites the entirety of WHITE ROOK 
    position.bb_pieces[Sides::BLACK][Pieces::PAWN] = 
        BitBoard((1 << 36) | (1 << 35));
    // bb_side occupancy (all white, all black) updated including all white/black positions 
    // https://rust-guide.com/en/documentation/iterators/fold -> reduce the elements of an iterator into a single value. same/similar to reduce() in java
    // should be moved inside the lib (eventually) -- into initalize_board function?
    position.bb_sides[Sides::WHITE].0 = position.bb_pieces[Sides::WHITE]
    .iter() 
    .fold(0u64, |acc, bb| acc | bb.0);
    position.bb_sides[Sides::BLACK].0 = position.bb_pieces[Sides::BLACK]
    .iter()
    .fold(0u64, |acc, bb| acc | bb.0);

    let mut game = Game::new(position);
    println!("Starting Color: {:?}", game.player_tracker());
    println!("Turn {}: {:?}", game.turn, game.player_tracker());
    /*game.turn_tracker();
    println!("Turn {}: {:?}", game.turn, game.player_tracker());*/
    /* game.turn_tracker();
    println!("Turn {}: {:?}", game.turn, game.player_tracker()); */

    // options: https://doc.rust-lang.org/std/option/ 
    match game.select_piece(square) {
        Ok(piece) => {
            println!("You selected: {:?} on square {}", piece, square);
            let moves = valid_moves(square, piece, &game.position);
            println!("Valid moves: {:?}", moves);
        }
        Err(msg) => println!("Selection failed: {}", msg),
    }
}
