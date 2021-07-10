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
    pub max_depth: Option<u64>,
    pub changes: [[bool; 3]; 3],
}

impl<'a> SearchInfo<Board<'a>> for BoardSearchExactChanges<'a> {
    fn is_goal(&self, board: &Board<'a>) -> bool {
        for x in 0..3 {
            for y in 0..3 {
                if !self.changes[x][y] && board.pieces[x][y] != self.goal.pieces[x][y] {
                    return false;
                }
                if self.changes[x][y] && board.pieces[x][y] == self.goal.pieces[x][y] {
                    return false;
                }
            }
        }
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
    pub moves: Vec<Vec<Direction>>
}

impl<'a> SearchInfo<Board<'a>> for BoardSearchCustomMoves<'a> {
    fn is_goal(&self, board: &Board<'a>) -> bool {
        self.delegate.is_goal(board)
    }

    fn heuristic(&self, state: &Board<'a>) -> u64 {
        self.delegate.heuristic(state)
    }

    fn expand_state(&self, board: &Board<'a>) -> Vec<Board<'a>> {
        self.moves.
            iter().
            map( |moves| board.apply_moves_to_empty_position_get_last(moves) ).
            map( |b| b.clone_with_pieceset(board.piece_set) ).
            collect()
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
    use Direction::*;

    vec![
        North, East, South, West, West, South, East, East, North, West, South, East, North, North,
        West, South, South, West, North, East,
    ]
}

pub fn moves_for_changes(diffs: [[bool; 3]; 3], max_depth: u64) -> Option<Vec<Direction>> {
    use crate::search::astar::*;
    let piece_set = PieceSet::from_piece(&Piece::seed());
    let board = Board::from_initial(&piece_set, piece_set.get_piece_index_of_initial_piece());
    let search = BoardSearchExactChanges {
        goal: board,
        max_depth: Some(max_depth),
        changes: diffs,
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

    #[test]
    fn is_goal() {
        let piece_set = PieceSet::from_piece(&Piece::seed());
        let search = BoardSearchAnyColor {};

        for i in 0..piece_set.pieces.len() {
            let board = Board::from_initial(&piece_set, i);
            assert!(search.is_goal(&board));
        }
    }

    #[test]
    fn root_is_goal() {
        let piece_set = PieceSet::from_piece(&Piece::seed());
        let board = Board::from_initial(&piece_set, 0);
        let search = BoardSearchAnyColor {};
        let (found, _, _) = a_star_search(board, &search);
        assert!(found.is_some());
        assert!(found.unwrap().borrow().state == board);
    }

    #[test]
    fn change_positions_1_8() {
        let moves = moves_for_change_1_8();

        let piece_set = PieceSet::from_piece(&Piece::seed());
        let board = Board::from_initial(&piece_set, piece_set.get_piece_index_of_initial_piece());

        let search_2_changes = BoardSearchSomeChanges {
            goal: board,
            max_depth: None,
            changes: 2,
        };

        let moved = board
            .apply_moves_to_empty_position(&moves)
            .last()
            .unwrap()
            .clone();

        println!("moved:\n{}", moved.ascii_art_string());

        assert!(search_2_changes.is_goal(&moved));
        println!("{}", moved.ascii_art_string());
        assert_ne!(board.pieces[1][0], moved.pieces[1][0]);
        assert_ne!(board.pieces[2][2], moved.pieces[2][2]);
    }

    #[test]
    fn moves_for_1_8_are_recognized() {
        let piece_set = PieceSet::from_piece(&Piece::seed());
        let board = Board::from_initial(&piece_set, piece_set.get_piece_index_of_initial_piece());

        let moves: Vec<Direction> = moves_for_change_1_8();

        let moved_boards = board.apply_moves_to_empty_position(&moves);

        let inferred_moves: Vec<Direction> = Board::infer_moves_to_empty_position(moved_boards);

        assert_eq!(moves.len(), inferred_moves.len());

        assert!(moves.iter().zip(inferred_moves).all(|(a, b)| *a == b))
    }

    #[test]
    fn change_positions_1_8_until_original() {
        let piece_set = PieceSet::from_piece(&Piece::seed());
        let board = Board::from_initial(&piece_set, piece_set.get_piece_index_of_initial_piece());

        let moves = moves_for_change_1_8();

        let mut moved = board
            .apply_moves_to_empty_position(&moves)
            .last()
            .unwrap()
            .clone();
        while moved.pieces != board.pieces {
            moved = moved
                .apply_moves_to_empty_position(&moves)
                .last()
                .unwrap()
                .clone();
            println!("moved");
        }
    }

    #[test]
    fn moves_for_two_changes() {
        fn diffs_for_changes(c1: usize, c2: usize) -> [[bool; 3]; 3] {
            let mut ret: [[bool; 3]; 3] = [
                [false, false, false],
                [false, false, false],
                [false, false, false],
            ];
            for x in 0..3 {
                for y in 0..3 {
                    let p = x * 3 + y;
                    ret[x][y] = p == c1 || p == c2;
                }
            }
            ret
        }

        // POSICIONES SIN ROTACIONES NI REFLEXIONES
        let changes = [[0, 1], [0, 2], [0, 5], [0, 8], [1, 3]];

        for [change_1, change_2] in changes.iter() {
            let diffs = diffs_for_changes(*change_1, *change_2);
            let moves = moves_for_changes(diffs, 25);
            match moves {
                None => {
                    println!("Diffs:{} {} no moves", change_1, change_2);
                }
                Some(moves) => {
                    println!("Diffs:{} {} Moves:{:?}", change_1, change_2, moves);
                }
            }
        }
    }

    #[test]
    fn search_on_scrambled_board() {
        fn search_with_step(step: usize) {
            let piece_set = PieceSet::from_piece(&Piece::seed());
            let board =
                Board::from_initial(&piece_set, piece_set.get_piece_index_of_initial_piece());
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
                .last()
                .unwrap()
                .clone();
            println!("moved_board:\n{}", moved_board.ascii_art_string());
            assert_eq!(moved_board, board);
        }

        let max = 40;

        for step in 10..max {
            search_with_step(step);
        }
    }
}
