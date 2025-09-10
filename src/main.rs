// name of library is defined in cargo.toml
use chess::*;
fn main() {
    let mut position = Position {
        bb_sides: [BitBoard(0), BitBoard(0)], //initalize bitboard
        bb_pieces: [[BitBoard(0); 6]; 2], 
    };

    // white pawn placed on 0
    position.bb_pieces[Sides::WHITE][Pieces::PAWN] = BitBoard(1 << 0); 
    // black queen placed on 3
    position.bb_pieces[Sides::BLACK][Pieces::QUEEN] = BitBoard(1 << 3); 

    let mut game = Game::new(position);
    println!("Starting Color: {:?}", game.player_tracker());
    println!("Turn {}: {:?}", game.turn, game.player_tracker());
    game.turn_tracker();
    println!("Turn {}: {:?}", game.turn, game.player_tracker());
    /* game.turn_tracker();
    println!("Turn {}: {:?}", game.turn, game.player_tracker()); */

    // options: https://doc.rust-lang.org/std/option/ 
    match get_piece_at(&game.position, 3) {
    Some(piece) => match game.select_piece(piece) {
        Ok(selected) => println!("There's a piece! You selected: {:?}", selected),
        Err(msg) => println!("There's a piece, but {}", msg),
    },
    None => println!("No piece here"),
    
}

}
