// name of library is defined in cargo.toml
use chess::*;

fn main() {
    let position = initialize_board();
    let mut game = Game::new(position);

    println!("Board Before move:");
    print_debug_board(&game.position);

    let moves_str: Vec<(&str, &str)> = vec![
        ("B2", "B4"),
        ("H7","H6"),
        ("B4","B5"),
        ("A7","A5"),
        ("B5","A6"),
        ("H6","H5"),
        ("A6","B7"),
        ("H5","H4"),
        ("B7","A8"),
        ("H4","H3"),
        ("A8", "B8"),
    ];

    let moves: Vec<(u8, u8)> = moves_str
    .iter()
    .filter_map(|(from, to)| {
        Some((
            square_to_index(from)?,
            square_to_index(to)?,
        ))
    })
    .collect();

    for (from, to) in moves {
        if execute_move(&mut game, from, to) {
            break; // stop processing remaining moves
        }
    }
}

/// Find index of the move that goes to `to_square`.
fn find_move_to(moves: &Vec<Move>, to_square: u8) -> Option<usize> {
    moves.iter().position(|m| m.to == to_square)
}
/// Execute the move from `from_square` to `to_square` (searches the valid_moves and uses make_move).
fn execute_move(game: &mut Game, from_square: u8, to_square: u8) -> bool {
    println!("Current Color: {:?}", game.player_tracker());
    match game.select_piece(from_square) {
        Ok(piece) => {
            println!("You selected: {:?} on square {}", piece, index_to_square(from_square));
            let moves = valid_moves(from_square, piece, &game.position);
            if moves.is_empty() {
                println!("No valid moves for this piece!");
                return false;
            }
            println!("Valid moves:");
            for (i, m) in moves.iter().enumerate() {
                println!("{}. {:?}", i, m);
            }
            println!("");

            // NOTE, being able to pick move id is important for special moves like promotion so this step should NOT always be left to computer.
            if let Some(idx) = find_move_to(&moves, to_square) {
                let chosen_move = moves[idx];
                println!("Choosing move index {} -> {:?}", idx, chosen_move);
                // just hnd over game and then let make_mvoe call position?
                match make_move(chosen_move, game) {
                    Ok(()) => {
                        println!("After move:");
                        print_debug_board(&game.position);
                        if game.is_over() {
                            println!("Game has ended: {:?}", game.result);
                            return true;
                        }
                    }
                    Err(e) => println!("Move failed: {}", e),
                }
            } else {
                println!("No valid move from {:?} to {:?} found.", index_to_square(from_square), index_to_square(to_square));
            }
        }
        Err(msg) => println!("Selection failed: {}", msg),
    }
    false
}