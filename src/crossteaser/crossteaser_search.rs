pub use crate::crossteaser::*;
use crate::search::*;

impl<'a> std::hash::Hash for Board<'a> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.pieces.hash(state);
    }
}

impl<'a> State for Board<'a> {}

#[derive(Debug)]
pub struct BoardSearchAnyColor {}
impl<'a> SearchInfo<Board<'a>> for BoardSearchAnyColor {
    fn is_goal(&self, board: &Board<'a>) -> bool {
        let pieces = board.pieces;
        let first_non_empty_piece = {
            if pieces[0][0] != Board::empty() {
                pieces[0][0]
            } else {
                pieces[0][1]
            }
        };

        for x in 0..3 {
            for y in 0..3 {
                if pieces[x][y] != Board::empty() && pieces[x][y] != first_non_empty_piece {
                    return false;
                }
            }
        }
        true
    }
    fn expand_state(&self, board: &Board<'a>) -> Vec<Board<'a>> {
        board.children_filtered()
    }
}

#[derive(Debug)]
pub struct BoardSearchWithGoal<'a> {
    pub goal: Board<'a>,
    pub max_depth: Option<u64>,
}

impl<'a> SearchInfo<Board<'a>> for BoardSearchWithGoal<'a> {
    fn is_goal(&self, board: &Board<'a>) -> bool {
        board.pieces == self.goal.pieces
    }

    fn expand_state(&self, board: &Board<'a>) -> Vec<Board<'a>> {
        board.children_filtered()
    }

    fn max_depth(&self) -> Option<u64> {
        self.max_depth
    }
}

#[derive(Debug)]
pub struct BoardSearchSomeChanges<'a> {
    pub goal: Board<'a>,
    pub max_depth: Option<u64>,
    pub changes: u8,
}

impl<'a> SearchInfo<Board<'a>> for BoardSearchSomeChanges<'a> {
    fn is_goal(&self, board: &Board<'a>) -> bool {
        if board.empty_coords() != self.goal.empty_coords() {
            return false;
        }
        let mut changes = 0;

        for x in 0..3 {
            for y in 0..3 {
                if board.pieces[x][y] != self.goal.pieces[x][y] {
                    changes += 1;
                }
            }
        }
        changes == self.changes
    }

    fn expand_state(&self, board: &Board<'a>) -> Vec<Board<'a>> {
        board.children_filtered()
    }

    fn max_depth(&self) -> Option<u64> {
        self.max_depth
    }
}

#[derive(Debug)]
pub struct BoardSearchExactChanges<'a> {
    pub goal: Board<'a>,
    pub fixed_piece_ids: Vec<usize>,
    pub max_depth: Option<u64>,
    pub changes: Vec<usize>,
}

impl<'a> SearchInfo<Board<'a>> for BoardSearchExactChanges<'a> {
    fn is_goal(&self, board: &Board<'a>) -> bool {
        for i in 0..9 {
            // HAY QUE MIRAR TAMBIEN LOS ID
            // es correcto si se cumple todo lo siguiente:
            // - si el índice tiene que cambiar, la pieza es distinta
            // - si el índice no tiene que cambiar, la pieza es la misma y su ID es de los que estaban en una posición que no iba a cambiar
            // por tanto es falso si:
            // - si el índice tiene que cambiar, la pieza no es distinta
            // - si el índice no tiene que cambiar, la pieza es distinta o su índice no es de los que no tienen que cambiar
            let is_change = self.changes.contains(&i);
            let is_fixed_piece_id = self.fixed_piece_ids.contains(&board.piece_id_from_index(i));
            let index_changed = self.goal.piece_from_index(i) == board.piece_from_index(i);
            if is_change && index_changed {
                return false;
            }
            if !is_change && !index_changed {
                return false;
            }
        }

        println!("Ahora falta comprobar que estos cambios son cíclicos sin cambiar nada más, espero que valga con los ID");
        true
    }

    fn heuristic(&self, _state: &Board<'a>) -> u64 {
        0
    }

    fn expand_state(&self, board: &Board<'a>) -> Vec<Board<'a>> {
        board.children_filtered()
    }

    fn max_depth(&self) -> Option<u64> {
        self.max_depth
    }
}

#[derive(Debug)]
pub struct BoardSearchCustomMoves<'a> {
    pub delegate: &'a dyn SearchInfo<Board<'a>>,
    pub moves: Vec<Vec<Direction>>,
}

impl<'a> SearchInfo<Board<'a>> for BoardSearchCustomMoves<'a> {
    fn is_goal(&self, board: &Board<'a>) -> bool {
        self.delegate.is_goal(board)
    }

    fn heuristic(&self, state: &Board<'a>) -> u64 {
        self.delegate.heuristic(state)
    }

    fn expand_state(&self, board: &Board<'a>) -> Vec<Board<'a>> {
        println!(
            "BoardSearchCustomMoves::expand_state:\n{}",
            board.ascii_art_string()
        );
        self.moves
            .iter()
            .map(|moves| board.apply_moves_to_empty_position_get_last(moves))
            .filter(|b| b.is_some())
            .map(|b| b.unwrap().clone_with_pieceset(board.piece_set))
            .collect()
    }

    fn max_depth(&self) -> Option<u64> {
        self.delegate.max_depth()
    }
}

pub fn scrambled_board<'a>(initial_board: &Board<'a>, steps: usize) -> Board<'a> {
    use rand::rngs::StdRng;
    use rand::Rng;
    use rand::SeedableRng;

    let mut board = initial_board.clone();
    let mut rng = StdRng::seed_from_u64(1);

    for _ in 0..steps {
        let children = board.children_filtered();
        let index = rng.gen_range(0, children.len());
        board = children[index];
    }
    board
}

pub fn moves_for_change_1_8() -> Vec<Direction> {
    moves_for_changes(vec![1, 8], 26).unwrap()
}

pub fn indexes_to_changes(indexes: &Vec<u64>) -> [[bool; 3]; 3] {
    let mut diffs: [[bool; 3]; 3] = [
        [false, false, false],
        [false, false, false],
        [false, false, false],
    ];
    for i in 0..3 {
        for j in 0..3 {
            let index = (i * 3 + j) as u64;
            diffs[i][j] = indexes.contains(&index);
        }
    }
    diffs
}

pub fn moves_for_changes(changes: Vec<usize>, max_depth: u64) -> Option<Vec<Direction>> {
    use crate::search::astar::*;
    let piece_set = PieceSet::from_piece(&Piece::seed());
    let board = Board::from_piece(&piece_set, piece_set.get_piece_index_of_initial_piece());
    let mut fixed_piece_ids: Vec<usize> = Vec::new();

    for i in 0..9 {
        if !changes.contains(&i) {
            fixed_piece_ids.push(board.piece_id_from_index(i))
        }
    }

    let search = BoardSearchExactChanges {
        goal: board,
        max_depth: Some(max_depth),
        changes: changes,
        fixed_piece_ids,
    };
    let (found, _, _) = a_star_search(board, &search);

    match found {
        None => None,
        Some(found) => {
            let to_root = root_path_state(&found);
            let moves = Board::infer_moves_to_empty_position(to_root);
            Some(moves)
        }
    }
}

#[cfg(test)]
mod tests {

    use crate::crossteaser::crossteaser_search::*;
    use crate::search::astar::*;
    use ntest::timeout;

    fn assert_moves<'a>(from: &Board<'a>, moves: &Vec<Direction>, to: Board) -> bool {
        let candidate = from.apply_moves_to_empty_position_get_last(moves);
        assert!(candidate.is_some());
        let candidate = candidate.unwrap();
        candidate.pieces == to.pieces
    }

    #[test]
    fn is_goal() {
        let piece_set = PieceSet::from_piece(&Piece::seed());
        let search = BoardSearchAnyColor {};

        for i in 0..piece_set.pieces.len() {
            let board = Board::from_piece(&piece_set, i);
            assert!(search.is_goal(&board));
        }
    }

    #[test]
    fn root_is_goal() {
        let piece_set = PieceSet::from_piece(&Piece::seed());
        let board = Board::from_piece(&piece_set, 0);
        let search = BoardSearchAnyColor {};
        let (found, _, _) = a_star_search(board, &search);
        assert!(found.is_some());
        assert!(found.unwrap().borrow().state == board);
    }

    #[test]
    #[timeout(10000)]
    fn change_positions_1_8() {
        let moves = moves_for_change_1_8();

        let piece_set = PieceSet::from_piece(&Piece::seed());
        let board = Board::from_piece(&piece_set, piece_set.get_piece_index_of_initial_piece());

        let search_2_changes = BoardSearchSomeChanges {
            goal: board,
            max_depth: None,
            changes: 2,
        };

        let moved = board
            .apply_moves_to_empty_position(&moves)
            .unwrap()
            .last()
            .unwrap()
            .clone();

        println!("moved:\n{}", moved.ascii_art_string());

        assert!(search_2_changes.is_goal(&moved));
        println!("board:\n{}", board.ascii_art_string());

        assert_ne!(board.piece_from_index(1), moved.piece_from_index(1));
        assert_ne!(board.piece_from_index(8), moved.piece_from_index(8));
    }

    #[test]
    #[timeout(10000)]
    fn moves_for_1_8_are_recognized() {
        let piece_set = PieceSet::from_piece(&Piece::seed());
        let board = Board::from_piece(&piece_set, piece_set.get_piece_index_of_initial_piece());

        let moves: Vec<Direction> = moves_for_change_1_8();

        let moved_boards = board.apply_moves_to_empty_position(&moves);

        let inferred_moves: Vec<Direction> =
            Board::infer_moves_to_empty_position(moved_boards.unwrap());

        assert_eq!(moves.len(), inferred_moves.len());

        assert!(moves.iter().zip(inferred_moves).all(|(a, b)| *a == b))
    }

    #[test]
    #[timeout(10000)]
    fn change_positions_1_8_until_original() {
        let piece_set = PieceSet::from_piece(&Piece::seed());
        let board = Board::from_piece(&piece_set, piece_set.get_piece_index_of_initial_piece());

        let moves = moves_for_change_1_8();

        let mut moved = board
            .apply_moves_to_empty_position(&moves)
            .unwrap()
            .last()
            .unwrap()
            .clone();
        while moved.pieces != board.pieces {
            moved = moved
                .apply_moves_to_empty_position(&moves)
                .unwrap()
                .last()
                .unwrap()
                .clone();
            println!("moved");
        }
    }

    #[test]
    #[timeout(10000)]
    fn moves_for_two_changes_possible() {
        // POSICIONES SIN ROTACIONES NI REFLEXIONES
        let possible_changes = [[0, 1], [0, 5], [1, 3]];
        let depth = 20;
        for [change_1, change_2] in possible_changes.iter() {
            let moves = moves_for_changes(vec![*change_1, *change_2], depth);
            assert!(moves.is_some());
            println!("Diffs:{} {} Moves:{:?}", change_1, change_2, moves.unwrap());
        }
    }

    #[test]
    #[timeout(100000)]
    fn find_moves_for_three_changes() {
        // POSICIONES SIN ROTACIONES NI REFLEXIONES
        let possible_changes = [
            [0, 1, 2],
            [0, 1, 3],
            [0, 1, 5],
            [0, 1, 6],
            [0, 1, 7],
            [0, 1, 8],
            [0, 2, 6],
        ];
        let depth = 20;
        for [change_1, change_2, change_3] in possible_changes.iter() {
            let moves = moves_for_changes(vec![*change_1, *change_2, *change_3], depth);
            if moves.is_some() {
                println!(
                    "Diffs:{} {} {} Moves:{:?}",
                    change_1,
                    change_2,
                    change_3,
                    moves.unwrap()
                );
            } else {
                println!(
                    "Not found for changes:{} {} {} Moves:{:?}",
                    change_1,
                    change_2,
                    change_3,
                    moves.unwrap()
                );
            }
        }
    }

    #[test]
    #[timeout(10000)]
    fn moves_for_two_changes_impossible() {
        // POSICIONES SIN ROTACIONES NI REFLEXIONES
        let impossible_changes = [[0, 2], [0, 8]];
        let depth = 26;
        for [change_1, change_2] in impossible_changes.iter() {
            let moves = moves_for_changes(vec![*change_1, *change_2], depth);
            assert!(moves.is_none());
            println!("Diffs:{} {} No moves", change_1, change_2);
        }
    }

    #[test]
    #[timeout(10000)]
    fn moves_for_two_changes_are_for_two_changes() {
        // POSICIONES SIN ROTACIONES NI REFLEXIONES
        let possible_changes = [[0, 1], [0, 5], [1, 3]];
        let depth = 26;
        for [change_1, change_2] in possible_changes.iter() {
            let changes = vec![*change_1, *change_2];

            let moves = moves_for_changes(changes.clone(), depth).unwrap();
            println!("Moves for {:?}:{:?}", changes, moves);

            let piece_set = PieceSet::from_piece(&Piece::seed());
            let board = Board::from_piece(&piece_set, piece_set.get_piece_index_of_initial_piece());
            let mut moved: Board = board.clone();
            for _i in 0..3 {
                let new_moved = moved
                    .apply_moves_to_empty_position_get_last(&moves)
                    .unwrap();
                moved = new_moved.clone_with_pieceset(moved.piece_set);

                println!("\n\nboard:\n{}", board.ascii_art_string());
                println!("\n\nmoved:\n{}", moved.ascii_art_string());

                for i in 0..9 {
                    if i == *change_1 || i == *change_2 {
                        assert_ne!(
                            board.piece_from_index(i as usize),
                            moved.piece_from_index(i as usize)
                        );
                    } else {
                        assert_eq!(
                            board.piece_from_index(i as usize),
                            moved.piece_from_index(i as usize)
                        );
                    }
                }
            }
        }
    }

    #[test]
    fn search_on_scrambled_board() {
        fn search_with_step(step: usize) {
            let piece_set = PieceSet::from_piece(&Piece::seed());
            let board = Board::from_piece(&piece_set, piece_set.get_piece_index_of_initial_piece());
            let scrambled = scrambled_board(&board, step);

            println!(
                "Probando con paso:{} -- board:{} -- scrambled:{}",
                step, board, scrambled
            );

            let search = BoardSearchAnyColor {};
            let (found, _, _) = a_star_search(scrambled, &search);
            let found = found.unwrap();
            assert!(found.borrow().state == board);

            let to_root = root_path_state(&found);
            to_root
                .iter()
                .for_each(|b| println!("{}\n\n", b.ascii_art_string()));

            assert!(to_root[0] == scrambled);
            assert!(to_root[to_root.len() - 1] == board);

            println!("INFER MOVES:");
            let moves = Board::infer_moves_to_empty_position(to_root);
            println!("moves:{:?}", moves);
            let moved_board = scrambled
                .apply_moves_to_empty_position(&moves)
                .unwrap()
                .last()
                .unwrap()
                .clone();
            println!("moved_board:\n{}", moved_board.ascii_art_string());
            assert_eq!(moved_board, board);
            assert!(assert_moves(&scrambled, &moves, board));
        }

        let max = 50;

        for step in 10..max {
            search_with_step(step);
        }
    }

    #[test]
    fn test_piece_id() {
        let piece_set = PieceSet::from_piece(&Piece::seed());
        let board = Board::from_piece(&piece_set, piece_set.get_piece_index_of_initial_piece());
        let id = board.piece_id_from_coords(1, 0);
        assert_ne!(id, Board::empty());
        assert_eq!(board.piece_id_from_coords(1, 1), Board::empty());

        let board = board.move_empty_position(Direction::North).unwrap();
        assert_eq!(board.piece_id_from_coords(1, 0), Board::empty());
        assert_eq!(id, board.piece_id_from_coords(1, 1));
    }
}
