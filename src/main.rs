// name of library is defined in cargo.toml
use chess::*;
fn main() {
    let mut position = Position {
        bb_sides: [BitBoard(0), BitBoard(0)], //initalize bitboard
        bb_pieces: [[BitBoard(0); 6]; 2], 
    };

    position.bb_pieces[Sides::WHITE][Pieces::PAWN] = BitBoard(1 << 0); 
    position.bb_pieces[Sides::BLACK][Pieces::QUEEN] = BitBoard(1 << 3); 

    let mut game = Game::new(position);
    println!("Turn {}: {:?}", game.turn, game.player_tracker());
    game.turn_tracker();
    println!("Turn {}: {:?}", game.turn, game.player_tracker());
    game.turn_tracker();
    println!("Turn {}: {:?}", game.turn, game.player_tracker());

    match get_piece_at(&position, 0) {
        Some(piece) => println!("Found piece: {:?}", piece),
        None => println!("No piece here"),
    }
}
