// name of library is defined in cargo.toml
use chess::*;

fn main() {
    // ----------------initializing s
    let mut position = Position {
        bb_sides: [BitBoard(0), BitBoard(1)], //initalize bitboard
        bb_pieces: [[BitBoard(0); 6]; 2],
    };
    let square: u8 = 8;
    position.bb_pieces[Sides::WHITE][Pieces::PAWN] = BitBoard(1 << square); // fyi this overwrites the entirety of WHITE ROOK board
    position.bb_pieces[Sides::BLACK][Pieces::PAWN] = BitBoard((1 << 17) | (1 << 35));

    position.bb_sides[Sides::WHITE].0 = position.bb_pieces[Sides::WHITE]
        .iter()
        .fold(0u64, |acc, bb| acc | bb.0);
    position.bb_sides[Sides::BLACK].0 = position.bb_pieces[Sides::BLACK]
        .iter()
        .fold(0u64, |acc, bb| acc | bb.0);

    // ----------------starts from here really
    let mut game = Game::new(position);
    
    let move_sequence = vec![
        (8, 2),  // white: select square 8, pick move 0 from moves Vec
        (35, 0), //black
        (17, 0), // white
    ];

    println!("Before move:");
    print_debug_board(&game.position);

    for (square, move_idx) in move_sequence {
        execute_move(&mut game, square, move_idx);
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
                        let color_code = if side == Sides::WHITE { "\x1b[31m" } else { "\x1b[32m" }; // red / green
                        let piece_char = match i {
                            0 => "P",
                            1 => "N",
                            2 => "B",
                            3 => "R",
                            4 => "Q",
                            5 => "K",
                            _ => "?",
                        };
                        output = format!("{}{}{}\x1b[0m", color_code, if side == Sides::WHITE { "W" } else { "B" }, piece_char);
                    }
                }
            }
            print!("{} ", output);
        }
        println!();
    }
    println!();
}

/// Finds piece given square, calculates possible moves, and executes move based on move_index (as listed in terminal)
fn execute_move(game: &mut Game, square: u8, move_index: usize) {
    println!("Current Color: {:?}", game.player_tracker());
    match game.select_piece(square) {
        Ok(piece) => {
            println!("You selected: {:?} on square {}", piece, square);
            let moves = valid_moves(square, piece, &game.position);
            if moves.is_empty() {
                println!("No valid moves for this piece!");
                return;
            }
            println!("Valid moves:");
            for (i, m) in moves.iter().enumerate() {
                println!("{}. {:?}", i, m);
            }
            println!("");
            let chosen_move = moves.get(move_index).expect("Invalid move index");
            match make_move(*chosen_move, &mut game.position) {
                Ok(()) => println!("Move executed successfully."),
                Err(e) => println!("Move failed: {}", e),
            }
            game.turn_tracker();
        }
        Err(msg) => println!("Selection failed: {}", msg),
    }

    println!("After move:");
    print_debug_board(&game.position);
}