use nego::core::{
    game::State, move_tab::LUTEntry, orientation::Orientation, pieces::PieceId, r#move::Move,
    square::Square,
};

fn mk_move(piece: PieceId, pos: (u8, u8), orientation: Orientation) -> Move {
    let entry = LUTEntry::lookup(
        piece.piece_type_id(),
        Square::from_indices(pos.0 as usize, pos.1 as usize),
        orientation,
    )
    .unwrap();
    Move::new(piece, entry)
}

fn play_moves(state: &mut State, moves: &[(PieceId, (u8, u8), Orientation)]) {
    moves.iter().for_each(|(a, b, c)| {
        state.place(mk_move(*a, *b, *c));
        state.current = state.current.next();
    });
}

#[test]
fn no_capture_boss() {
    use Orientation::*;
    use PieceId::*;

    let mut state = State::new();
    let first_boss = mk_move(Boss, (0, 0), S);
    state.place(first_boss);
    state.current = state.current.next();
    state.dump();
    let occupied = state.board.black.occupied;

    play_moves(
        &mut state,
        &[
            (Boss, (0, 2), S), // W
            (Mame, (5, 5), S), // B
        ],
    );

    println!("before placement:");
    state.dump();
    state.place(mk_move(Nobi, (2, 0), W)); // W
    println!("after placement:");
    state.dump();

    assert!(state.board.black.occupied & occupied == occupied);
    assert_eq!(state.board.black.move_list[0].get_piece(), Boss);
}

#[test]
fn snapshot() {
    use rand::{seq::SliceRandom, SeedableRng};
    let mut rng = rand::rngs::StdRng::seed_from_u64(123);

    for _ in 0..20 {
        let mut state = State::new();
        let mut moves = Vec::new();
        let mut history = Vec::new();
        loop {
            history.push(state.clone());
            moves.truncate(0);
            state.get_moves(&mut moves);
            if moves.is_empty() {
                break;
            }
            let m = *moves.choose(&mut rng).unwrap();
            state.apply(m);
        }

        insta::assert_debug_snapshot!(history);
    }
}
