// name of library is defined in cargo.toml
use chess::*;

fn main() {
    // ---------------initializing s
    let mut position = Position {
        bb_sides: [BitBoard(0), BitBoard(1)],
        bb_pieces: [[BitBoard(0); 6]; 2],
    };
    //position.bb_pieces[Sides::WHITE][Pieces::ROOK] = BitBoard(1 << 0);  
    position.bb_pieces[Sides::WHITE][Pieces::QUEEN] = BitBoard(1 << 45 | 1 << 0 ); 
    position.bb_pieces[Sides::BLACK][Pieces::KING] = BitBoard(1 << 63);
    position.bb_pieces[Sides::BLACK][Pieces::ROOK] = BitBoard(1 << 9);

    position.bb_sides[Sides::WHITE].0 = position.bb_pieces[Sides::WHITE]
        .iter()
        .fold(0u64, |acc, bb| acc | bb.0);
    position.bb_sides[Sides::BLACK].0 = position.bb_pieces[Sides::BLACK]
        .iter()
        .fold(0u64, |acc, bb| acc | bb.0);

    // --------------main.rs
    let mut game = Game::new(position);

    println!("Board Before move:");
    print_debug_board(&game.position);

    execute_move(&mut game, 1, 7); // white picks invalid move
    execute_move(&mut game, 0, 7); // white tries again and succeeds
    execute_move(&mut game, 9, 15); // black plays
    execute_move(&mut game, 7, 15); 
    execute_move(&mut game, 63, 55); // black attempts illegal king move (would result in check)
    execute_move(&mut game, 63, 62); 
    execute_move(&mut game, 15, 43); 
}

/// Find index of the move that goes to `to_square`.
fn find_move_to(moves: &Vec<Move>, to_square: u8) -> Option<usize> {
    moves.iter().position(|m| m.to == to_square)
}
/// Execute the move from `from_square` to `to_square` (searches the valid_moves and uses make_move).
fn execute_move(game: &mut Game, from_square: u8, to_square: u8) {
    println!("Current Color: {:?}", game.player_tracker());
    match game.select_piece(from_square) {
        Ok(piece) => {
            println!("You selected: {:?} on square {}", piece, from_square);
            let moves = valid_moves(from_square, piece, &game.position);
            if moves.is_empty() {
                println!("No valid moves for this piece!");
                return;
            }
            println!("Valid moves:");
            for (i, m) in moves.iter().enumerate() {
                println!("{}. {:?}", i, m);
            }
            println!("");

            if let Some(idx) = find_move_to(&moves, to_square) {
                let chosen_move = moves[idx];
                println!("Choosing move index {} -> {:?}", idx, chosen_move);
                // just hnd over game and then let make_mvoe call position?
                match make_move(chosen_move, game) {
                    Ok(()) => {
                        println!("After move:");
                        print_debug_board(&game.position);
                    }
                    Err(e) => println!("Move failed: {}", e),
                }
            } else {
                println!("No valid move from {} to {} found.", from_square, to_square);
            }
        }
        Err(msg) => println!("Selection failed: {}", msg),
    }
}


/// Prints the state of the board with all sides.
/// Made up of numbers (bit indices 0..63) and letters (where "WP" = "White Pawn" and "BN" = "Black Knight")
pub fn print_debug_board(position: &Position) {
    for row in (0..8).rev() {
        for col in 0..8 {
            let square: i32 = row * 8 + col;
            let mask = 1u64 << square;
            let mut output = format!("{:02}", square); // default: bit index

            for side in [Sides::WHITE, Sides::BLACK] {
                for (i, bb) in position.bb_pieces[side].iter().enumerate() {
                    if (bb.0 & mask) != 0 {
                        let color_code = if side == Sides::WHITE {
                            "\x1b[31m"
                        } else {
                            "\x1b[32m"
                        }; // red / green
                        let piece_char = match i {
                            0 => "P",
                            1 => "N",
                            2 => "B",
                            3 => "R",
                            4 => "Q",
                            5 => "K",
                            _ => "?",
                        };
                        output = format!(
                            "{}{}{}\x1b[0m",
                            color_code,
                            if side == Sides::WHITE { "W" } else { "B" },
                            piece_char
                        );
                    }
                }
            }
            print!("{} ", output);
        }
        println!();
    }
    println!();
}
