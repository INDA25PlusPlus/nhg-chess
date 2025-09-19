use crate::game::{Game, GameResult};
use crate::moves::{Move, valid_moves};
use crate::piece::{Color, Piece};
use crate::position::{Pieces, Position, Sides};

// see: https://www.chessprogramming.org/Bitboard_Serialization

/// Attempts to make a move in the given game.
///
/// # Behavior
/// - Rejects moves not found in [`valid_moves`].
/// - Rejects moves that would leave the mover’s own king in check.
/// - Otherwise, commits the move to the game state.
/// - Updates castling rights and en passant.
/// - Prints if the enemy king is in check, checkmate, or stalemate.
/// - Advances the turn if the game is not over.
///
/// # Arguments
/// * `m` - The move to attempt.
/// * `game` - The mutable game state to apply the move to.
///
/// # Errors
/// Returns `Err(String)` if the move is illegal.
///
/// # Returns
/// `Ok(())` if the move was applied successfully.
pub fn make_move(m: Move, game: &mut Game) -> Result<(), String> {
    let position = &mut game.position;
    let valid = valid_moves(m.from, m.piece, position);
    if !valid.contains(&m) {
        return Err("Illegal move (not in generated valid moves)".to_string());
    }

    // simulate on a clone to check if this move leaves current player's king in check
    let mut test_pos = position.clone();
    apply_move_unchecked(m, &mut test_pos);
    if is_checked(m.piece.color(), &test_pos) {
        return Err("Illegal move: would leave your king in check".to_string());
    }

    // commit to real position
    apply_move_unchecked(m, position);
    update_castling_rights(m, position);

    // check if opponent king is in check
    let enemy_color = match m.piece.color() {
        Color::White => Color::Black,
        Color::Black => Color::White,
    };

    if is_checked(enemy_color, position) {
        println!("{:?} king is in check", enemy_color);
    }
    if is_checkmated(enemy_color, &position) {
        println!("{:?} is checkmated.", enemy_color);
        game.result = GameResult::Checkmate(enemy_color);
        return Ok(());
    } else if is_stalemated(enemy_color, &position) {
        println!("Stalemate! It's a draw.");
        game.result = GameResult::Stalemate;
        return Ok(());
    }
    //println!("En Passant: {:?}", position.en_passant);
    game.turn_tracker();
    Ok(())
}

/// Updates castling rights in the given position after a move.
///
/// Castling rights are revoked when a king or rook moves from its initial square.
///
/// # Arguments
/// * `m` - The move to check.
/// * `position` - The mutable board state to update.
fn update_castling_rights(m: Move, position: &mut Position) {
    match m.piece {
        Piece::King(Color::White) => position.castling_rights.white_king_moved = true,
        Piece::King(Color::Black) => position.castling_rights.black_king_moved = true,
        Piece::Rook(Color::White) => {
            if m.from == 0 {
                position.castling_rights.white_queenside_rook_moved = true;
            } // can this cause a pooblem if it mvoes multiple times from the same spot? true + true = ...?
            if m.from == 7 {
                position.castling_rights.white_kingside_rook_moved = true;
            }
        }
        Piece::Rook(Color::Black) => {
            if m.from == 56 {
                position.castling_rights.black_queenside_rook_moved = true;
            }
            if m.from == 63 {
                position.castling_rights.black_kingside_rook_moved = true;
            }
        }
        _ => {}
    }
}

/// Applies a move directly to the given position without legality checks.
///
/// This function updates side and piece bitboards, handles captures,
/// castling, promotions, and en passant.
///
/// # Arguments
/// * `m` - The move to apply.
/// * `position` - The mutable board state to update.
pub fn apply_move_unchecked(m: Move, position: &mut Position) {
    let color = m.piece.color();
    let friendly_index = match color {
        Color::White => Sides::WHITE,
        Color::Black => Sides::BLACK,
    };
    let enemy_index = if friendly_index == Sides::WHITE {
        Sides::BLACK
    } else {
        Sides::WHITE
    };

    let from_mask: u64 = 1u64 << m.from;
    let to_mask: u64 = 1u64 << m.to;

    position.bb_sides[friendly_index].0 &= !from_mask;

    if m.promoted_from_pawn {
        //println!("it's a promotion, ja");
        position.bb_pieces[friendly_index][Pieces::PAWN].0 &= !from_mask;
    } else {
        let piece_index = match m.piece {
            Piece::Pawn(_) => Pieces::PAWN,
            Piece::Knight(_) => Pieces::KNIGHT,
            Piece::Bishop(_) => Pieces::BISHOP,
            Piece::Rook(_) => Pieces::ROOK,
            Piece::Queen(_) => Pieces::QUEEN,
            Piece::King(_) => Pieces::KING,
        };
        position.bb_pieces[friendly_index][piece_index].0 &= !from_mask;
    }
    // if 'to' square contains an enemy, remove that enemy piece
    if (position.bb_sides[enemy_index].0 & to_mask) != 0 {
        // remove from enemy SIDE occupancy
        position.bb_sides[enemy_index].0 &= !to_mask;
        // find which enemy PIECE LAYER has the bit and clear it
        for i in 0..6 {
            if (position.bb_pieces[enemy_index][i].0 & to_mask) != 0 {
                position.bb_pieces[enemy_index][i].0 &= !to_mask;
                break;
            }
        }
    }
    // Castling: king moves 2 squares horizontally
    if let Piece::King(color) = m.piece {
        if (m.from as i8 - m.to as i8).abs() == 2 {
            let (rook_from, rook_to) = match (color, m.to) {
                (Color::Black, 62) => (63, 61),
                (Color::Black, 58) => (56, 59),
                (Color::White, 6) => (7, 5), 
                (Color::White, 2) => (0, 3),
                _ => (0, 0),
            };

            let rook_mask_from = 1u64 << rook_from;
            let rook_mask_to = 1u64 << rook_to;
            // ("rook from {}, to {}", rook_from, rook_to);

            // remove rook from original square
            position.bb_sides[friendly_index].0 &= !rook_mask_from;
            position.bb_pieces[friendly_index][Pieces::ROOK].0 &= !rook_mask_from;

            // place rook on new square
            position.bb_sides[friendly_index].0 |= rook_mask_to;
            position.bb_pieces[friendly_index][Pieces::ROOK].0 |= rook_mask_to;
        }
    }
    if let Piece::Pawn(pawn_color) = m.piece {
        let dir = match pawn_color {
            Color::White => 8,
            Color::Black => -8,
        };

        // check if this move is an en passant capture 
        if let Some(ep_square) = position.en_passant {
            if m.to == ep_square {
                let captured_pawn_square = (ep_square as i8 - dir) as u8;
                let captured_mask = 1u64 << captured_pawn_square;

                position.bb_sides[enemy_index].0 &= !captured_mask;
                position.bb_pieces[enemy_index][Pieces::PAWN].0 &= !captured_mask;
            }
        }
        position.en_passant = None;
        if (m.from as i8 + 2 * dir) == m.to as i8 {
            let ep_square = (m.from as i8 + dir) as u8;
            position.en_passant = Some(ep_square);
            //println!("En passant square {}", ep_square);
        }
    }

    let piece_index = if m.promoted_from_pawn {
        // The piece is the promoted piece
        match m.piece {
            Piece::Queen(_) => Pieces::QUEEN,
            Piece::Rook(_) => Pieces::ROOK,
            Piece::Bishop(_) => Pieces::BISHOP,
            Piece::Knight(_) => Pieces::KNIGHT,
            _ => Pieces::PAWN,
        }
    } else {
        match m.piece {
            Piece::Pawn(_) => Pieces::PAWN,
            Piece::Knight(_) => Pieces::KNIGHT,
            Piece::Bishop(_) => Pieces::BISHOP,
            Piece::Rook(_) => Pieces::ROOK,
            Piece::Queen(_) => Pieces::QUEEN,
            Piece::King(_) => Pieces::KING,
        }
    };

    position.bb_sides[friendly_index].0 |= to_mask;
    position.bb_pieces[friendly_index][piece_index].0 |= to_mask;
}

/// Returns `true` if the given color’s king is in check.
///
/// A king is considered checked if any opposing piece
/// has a legal attack on its square.
///
/// # Arguments
/// * `color` - The side to check for.
/// * `position` - The board state.
///
/// # Returns
/// `true` if the king is attacked, otherwise `false`.
pub fn is_checked(color: Color, position: &Position) -> bool {
    //println!("checking on the {:?} king! you alright?", color);
    let (friendly_index, enemy_index) = match color {
        Color::White => (Sides::WHITE, Sides::BLACK),
        Color::Black => (Sides::BLACK, Sides::WHITE),
    };
    let king_bb = position.bb_pieces[friendly_index][Pieces::KING].0;
    if king_bb == 0 {
        return false;
        // panic!("No king found for {:?} in position!", color);        add panic in final version
    }
    let king_sq = king_bb.trailing_zeros() as u8;
    let enemy_color = match color {
        Color::White => Color::Black,
        Color::Black => Color::White,
    };

    // iterate all enemy piece bitboards; generate moves and see if any threaten king
    for piece_type in 0..6 {
        let mut bb = position.bb_pieces[enemy_index][piece_type].0;
        while bb != 0 {
            let from = bb.trailing_zeros() as u8;
            bb &= bb - 1; // pop least significant bit

            let piece = match piece_type {
                Pieces::PAWN => Piece::Pawn(enemy_color),
                Pieces::KNIGHT => Piece::Knight(enemy_color),
                Pieces::BISHOP => Piece::Bishop(enemy_color),
                Pieces::ROOK => Piece::Rook(enemy_color),
                Pieces::QUEEN => Piece::Queen(enemy_color),
                Pieces::KING => Piece::King(enemy_color),
                _ => unreachable!(),
            };

            let moves = valid_moves(from, piece, position);
            if moves.iter().any(|mv| mv.to == king_sq) {
                return true;
            }
        }
    }

    false
}

/// Generates all legal moves for the given color.
///
/// Pseudo-legal moves are generated with [`valid_moves`],
/// then filtered to exclude moves that leave the king in check.
///
/// # Arguments
/// * `color` - The side to generate moves for.
/// * `position` - The board state.
///
/// # Returns
/// A vector of all legal moves available to `color`.
pub fn legal_moves(color: Color, position: &Position) -> Vec<Move> {
    let mut result = Vec::new();

    // find all friendly pieces
    let side_index = match color {
        Color::White => Sides::WHITE,
        Color::Black => Sides::BLACK,
    };

    for piece_type in 0..6 {
        let mut bb = position.bb_pieces[side_index][piece_type].0;
        while bb != 0 {
            let from = bb.trailing_zeros() as u8;
            bb &= bb - 1;

            let piece = match piece_type {
                Pieces::PAWN => Piece::Pawn(color),
                Pieces::KNIGHT => Piece::Knight(color),
                Pieces::BISHOP => Piece::Bishop(color),
                Pieces::ROOK => Piece::Rook(color),
                Pieces::QUEEN => Piece::Queen(color),
                Pieces::KING => Piece::King(color),
                _ => unreachable!(),
            };

            let pseudo_moves = valid_moves(from, piece, position);

            // filter out moves that leave king in check
            for m in pseudo_moves {
                let mut test_pos = position.clone();
                apply_move_unchecked(m, &mut test_pos);
                if !is_checked(color, &test_pos) {
                    result.push(m);
                }
            }
        }
    }
    result
}

/// Returns `true` if the given color is checkmated.
///
/// A side is checkmated if its king is in check and it has no legal moves.
///
/// # Arguments
/// * `color` - The side to test.
/// * `position` - The board state.
pub fn is_checkmated(color: Color, position: &Position) -> bool {
    //println!("Legal moves: {:?}", legal_moves(color, position));
    is_checked(color, position) && legal_moves(color, position).is_empty()
}


/// Returns `true` if the given color is stalemated.
///
/// A side is stalemated if its king is not in check and it has no legal moves.
///
/// # Arguments
/// * `color` - The side to test.
/// * `position` - The board state.
pub fn is_stalemated(color: Color, position: &Position) -> bool {
    !is_checked(color, position) && legal_moves(color, position).is_empty()
}
