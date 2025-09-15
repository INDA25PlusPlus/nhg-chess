// name of library is defined in cargo.toml
use chess::*;

fn main() {
    let mut position = Position {
        bb_sides: [BitBoard(0), BitBoard(0)], //initalize bitboard
        bb_pieces: [[BitBoard(0); 6]; 2], 
    };

    // white pawn placed on 0
    // position.bb_pieces[Sides::WHITE][Pieces::PAWN] = BitBoard(1 << 0); 
    // black KNIGHT placed on 3
    let square: u8 = 27;
    position.bb_pieces[Sides::BLACK][Pieces::KNIGHT] = BitBoard(1 << square); 

    let mut game = Game::new(position);
    println!("Starting Color: {:?}", game.player_tracker());
    println!("Turn {}: {:?}", game.turn, game.player_tracker());
    game.turn_tracker();
    println!("Turn {}: {:?}", game.turn, game.player_tracker());
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
