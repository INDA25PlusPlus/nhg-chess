use nhg_chess::{
    initialize_board,
    square_to_index,
    valid_moves,
    make_move,
    Game,
};
use nhg_chess::position::get_piece_at;
use nhg_chess::piece::Color;

#[test]
fn test_initial_board_has_32_pieces() {
    let pos = initialize_board();
    let mut count = 0;
    for sq in 0..64 {
        if get_piece_at(&pos, sq).is_some() {
            count += 1;
        }
    }
    assert_eq!(count, 32);
}

#[test]
fn test_white_to_move_first() {
    let pos = initialize_board();
    let game = Game::new(pos);
    assert_eq!(game.player_tracker(), Color::White);
}

#[test]
fn test_pawn_has_two_initial_moves() {
    let pos = initialize_board();
    let mut game = Game::new(pos);

    let from = square_to_index("E2").unwrap();
    let piece = game.select_piece(from).unwrap();
    let moves = valid_moves(from, piece, &game.position);

    let to_e3 = square_to_index("E3").unwrap();
    let to_e4 = square_to_index("E4").unwrap();
    let targets: Vec<u8> = moves.iter().map(|m| m.to).collect();

    assert!(targets.contains(&to_e3));
    assert!(targets.contains(&to_e4));
}

#[test]
fn test_make_move_updates_turn() {
    let pos = initialize_board();
    let mut game = Game::new(pos);

    let from = square_to_index("E2").unwrap();
    let piece = game.select_piece(from).unwrap();
    let moves = valid_moves(from, piece, &game.position);
    let to_e4 = square_to_index("E4").unwrap();
    let mv = moves.into_iter().find(|m| m.to == to_e4).unwrap();

    make_move(mv, &mut game).unwrap();

    assert_eq!(game.player_tracker(), Color::Black);
}

#[test]
fn test_en_passant_capture() {
    let pos = initialize_board();
    let mut game = Game::new(pos);
    // dummy example. 
    let seq = [
        ("H2","H3"),
        ("A7","A5"),
        ("H3","H4"),
        ("B7","B5"),
        ("H4","H5"),
        ("B5","B4"),
        ("A2","A4"),
        ("B4","A3"), // en passant capture
    ];

    for (from_s, to_s) in seq {
        let from = square_to_index(from_s).unwrap();
        let to = square_to_index(to_s).unwrap();

        let piece = game.select_piece(from).unwrap();
        let moves = valid_moves(from, piece, &game.position);

        let mv = moves.into_iter().find(|m| m.to == to)
            .unwrap_or_else(|| panic!("Move {from_s}->{to_s} not found in valid moves"));
        make_move(mv, &mut game).unwrap();
    }
    assert!(true);
}

