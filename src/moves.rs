use crate::piece::{Piece, Color};
use crate::position::Position;

#[derive(Debug)]
pub struct Move {
    pub from: u8,
    pub to: u8,
    pub piece: Piece,
}

pub fn valid_moves(from: u8, piece: Piece, position: &Position) -> Vec<Move> {
    match piece {
        Piece::Knight(_) => valid_knight_moves(from, piece, position),
        Piece::Pawn(_) => valid_pawn_moves(from, piece, position),
        Piece::Bishop(_) => valid_bishop_moves(from, piece, position),
        Piece::Rook(_) => valid_rook_moves(from, piece, position),
        Piece::Queen(_) => todo!("Implement queen moves"),
        Piece::King(_) => todo!("Implement king moves"),
    }
}

/// Get index of BitBoard side (bb_sides), where 0 indicates White, and 1 indicates Black.
pub fn piece_indexes(piece: Piece) -> (usize, usize) {
    match piece {
        Piece::Bishop(Color::White)
        | Piece::Knight(Color::White)
        | Piece::Pawn(Color::White)
        | Piece::Rook(Color::White)
        | Piece::Queen(Color::White)
        | Piece::King(Color::White) => (0, 1),

        Piece::Bishop(Color::Black)
        | Piece::Knight(Color::Black)
        | Piece::Pawn(Color::Black)
        | Piece::Rook(Color::Black)
        | Piece::Queen(Color::Black)
        | Piece::King(Color::Black) => (1, 0),
    }
}

/* 
Bitboard set-up:
8 | 56 57 58 59 60 61 62 63
7 | 48 49 50 51 52 53 54 55
6 | 40 41 42 43 44 45 46 47
5 | 32 33 34 35 36 37 38 39
4 | 24 25 26 27 28 29 30 31
3 | 16 17 18 19 20 21 22 23
2 |  8  9 10 11 12 13 14 15
1 |  0  1  2  3  4  5  6  7
    a  b  c  d  e  f  g  h
*/

/*
Pieces with normal moves only
- Knight X
- Bishop X
- Rook
- Queen

Pieces with special moves
- Pawn !
Promotion: when it reaches the last rank (rank 8 for White, rank 1 for Black).
En passant: capturing a pawn that just moved two squares.

- King
Castling: moves two squares left or right if rook + king haven’t moved, no check along the path.
*/

pub fn valid_knight_moves(from: u8, piece: Piece, position: &Position) -> Vec<Move> {
    // see knight-offset.jpg
    let knight_offsets: [i8; 8] = [17, 15, 10, 6, -17, -15, -10, -6];
    let mut moves = Vec::new();

    // Gives column as a value of an integer, i.e. 0 = A, 1 = B, .... 7 = H
    let from_column = (from % 8) as i8;
    // Gives row as value of an integer, i.e. 0 = row 1, 1 = row 2 ... 7 = row 8
    let from_row = (from / 8) as i8;

    for &offset in &knight_offsets {
        let target = from as i8 + offset;

        // stay inside board
        if target < 0 || target >= 63 {
            continue;
        }

        let target_column = (target % 8) as i8;
        let target_row = (target / 8) as i8;

        // horse cannot move greater than 2 squares in one direction 
        if (from_column - target_column).abs() > 2 || (from_row - target_row).abs() > 2 {
            continue;
        }

        let spotlight = 1u64 << target;

        // cannot land on own piece
        // move this function elsewhere - MERGE WITH CHECK_COLOR
        let side_index = match piece {
            Piece::Pawn(c)
            | Piece::Rook(c)
            | Piece::Knight(c)
            | Piece::Bishop(c)
            | Piece::Queen(c)
            | Piece::King(c) => match c {
                Color::White => 0,
                Color::Black => 1,
            },
        };

        if (position.bb_sides[side_index].0 & spotlight) != 0 {
            continue;
        }

        moves.push(Move {
            from,
            to: target as u8,
            piece,
        });
    }
    moves
}


// representing a large range of moves such as this? 
pub fn valid_bishop_moves(from: u8, piece: Piece, position: &Position) -> Vec<Move> {
    let mut moves = Vec::new();

    let (own_index, enemy_index) = piece_indexes(piece);
    
    let directions: [i8; 4] = [7, 9, -7, -9];
    for &dir in &directions {
        let mut target = from as i8;

        loop {
            target += dir;
            if target < 0 || target >= 64 {
                break;
            }

            let target_row = target / 8;
            let target_col = target % 8;
            let from_row = from as i8 / 8;
            let from_col = from as i8 % 8;

            if (target_row - from_row).abs() != (target_col - from_col).abs() {
                break; // not diagonal anymore → wrapped
            }

            let spotlight = 1u64 << target;
            // own piece? stop
            if (position.bb_sides[own_index].0 & spotlight) != 0 {
                break;
            }
            // enemy piece? push move, then stop
            if (position.bb_sides[enemy_index].0 & spotlight) != 0 {
                moves.push(Move { from, to: target as u8, piece });
                break;
            }
            moves.push(Move { from, to: target as u8, piece });
        }
    }
    moves
}

pub fn valid_rook_moves(from: u8, piece: Piece, position: &Position) -> Vec<Move> {
    let mut moves = Vec::new();

    let (own_index, enemy_index) = piece_indexes(piece);
    
    let directions: [i8; 4] = [1, 8, -1, -8];
    for &dir in &directions {
        let mut target = from as i8;

        loop {
            target += dir;
            if target < 0 || target >= 64 {
                break;
            }

            let target_row = target / 8;
            let target_col = target % 8;
            let from_row = from as i8 / 8;
            let from_col = from as i8 % 8;

            if target_row != from_row && target_col != from_col {
                break;
            }
            let spotlight = 1u64 << target;
            // own piece? stop
            if (position.bb_sides[own_index].0 & spotlight) != 0 {
                break;
            }
            // enemy piece? push move, then stop
            if (position.bb_sides[enemy_index].0 & spotlight) != 0 {
                moves.push(Move { from, to: target as u8, piece });
                break;
            }
            moves.push(Move { from, to: target as u8, piece });
        }
    }
    moves
}

pub fn valid_pawn_moves(from: u8, piece: Piece, position: &Position) -> Vec<Move> {
    let mut moves = Vec::new();

    let (dir, start_row) = match piece {
        Piece::Pawn(Color::White) => (8, 1), // white moves up
        Piece::Pawn(Color::Black) => (-8, 6), // black moves down
        _ => return moves, 
    };
    // own_index unused
    let (_own_index, enemy_index) = piece_indexes(piece);

    // reducing this repetition?
    let from_row = (from / 8) as i8;
    let from_col = (from % 8) as i8;

    let forward1 = from as i8 + dir;
    if forward1 >= 0 && forward1 < 64 {
        let spotlight = 1u64 << forward1;
        if (position.bb_sides[0].0 & spotlight == 0) && (position.bb_sides[1].0 & spotlight == 0) {
            moves.push(Move { from, to: forward1 as u8, piece });

            // 2-squares forward from starting row
            if from_row == start_row {
                let forward2 = forward1 + dir; // i.e. dir+dir
                if forward2 >= 0 && forward2 < 64 {
                    let spotlight2 = 1u64 << forward2;
                    if (position.bb_sides[0].0 & spotlight2 == 0) && (position.bb_sides[1].0 & spotlight2 == 0) {
                        moves.push(Move { from, to: forward2 as u8, piece });
                    }
                }
            }
        }
    }
    // Captures
    for &diag in &[dir - 1, dir + 1] {
        let target = from as i8 + diag;
        if target < 0 || target >= 64 {
            continue;
        }

        let target_col = (target % 8) as i8;
        // cannot wrap around horizontally
        if (target_col - from_col).abs() != 1 {
            continue;
        }

        let spotlight = 1u64 << target;
        println!("Enemy spotlight: {:064b}", spotlight);
        if (position.bb_sides[enemy_index].0 & spotlight) != 0 {
            println!("There's an enemy!! aaa");
            moves.push(Move { from, to: target as u8, piece });
        }
        // indicating in move that move is a capture?
    }
    moves
}


