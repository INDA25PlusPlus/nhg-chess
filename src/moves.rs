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
        Piece::Pawn(_) => todo!("Implement pawn moves"),
        Piece::Bishop(_) => todo!("Implement bishop moves"),
        Piece::Rook(_) => todo!("Implement rook moves"),
        Piece::Queen(_) => todo!("Implement queen moves"),
        Piece::King(_) => todo!("Implement king moves"),
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

