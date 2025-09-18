use crate::moves::{Move, valid_moves};
use crate::piece::{Color, Piece};
use crate::position::{Pieces, Position, Sides};
use crate::game::{Game, GameResult};

// see: https://www.chessprogramming.org/Bitboard_Serialization

/// Attempt and validate a move.
/// - rejects moves that are not in `valid_moves`
/// - rejects moves that would leave the mover's own king in check
/// - else, commits the move and prints if the enemy king is in check
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
    game.turn_tracker();
    Ok(())
}

/// Apply the move to `position` (across bitboards)
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

    let piece_index = match m.piece {
        Piece::Pawn(_) => Pieces::PAWN,
        Piece::Knight(_) => Pieces::KNIGHT,
        Piece::Bishop(_) => Pieces::BISHOP,
        Piece::Rook(_) => Pieces::ROOK,
        Piece::Queen(_) => Pieces::QUEEN,
        Piece::King(_) => Pieces::KING,
    };

    // remove piece from the 'from' square in both friendly SIDE & PIECE LAYER
    position.bb_sides[friendly_index].0 &= !from_mask;
    position.bb_pieces[friendly_index][piece_index].0 &= !from_mask;

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
    // update friendly SIDE occupancy and that PIECE LAYER
    position.bb_sides[friendly_index].0 |= to_mask;
    position.bb_pieces[friendly_index][piece_index].0 |= to_mask;
}

/// Return true if `color`'s king is currently attacked (i.e. in check).
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

pub fn is_checkmated(color: Color, position: &Position) -> bool {
    //println!("Legal moves: {:?}", legal_moves(color, position));
    is_checked(color, position) && legal_moves(color, position).is_empty()
}

pub fn is_stalemated(color: Color, position: &Position) -> bool {
    !is_checked(color, position) && legal_moves(color, position).is_empty()
}
