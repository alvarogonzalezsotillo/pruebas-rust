
pub use crate::crossteaser::*;
use crate::search::*;


impl <'a> std::hash::Hash for Board<'a>{
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.pieces.hash(state);
    }    
}

impl <'a> State for Board<'a>{
}




#[derive(Debug)]
pub struct BoardSearchAnyColor{
}

impl <'a> SearchInfo<Board<'a>> for BoardSearchAnyColor{

    fn is_goal(&self, board: &Board<'a> ) -> bool {
        let pieces = board.pieces;
        let first_non_empty_piece = {
            if pieces[0][0] != Board::empty(){
                pieces[0][0]
            }
            else{
                pieces[0][1]
            }
        };

        for x in 0..3{
            for y in 0..3{
                if pieces[x][y] != Board::empty() && pieces[x][y] != first_non_empty_piece{
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
pub struct BoardSearchWithGoal<'a>{
    pub goal: Board<'a>,
    pub max_depth: Option<u64>
}

impl <'a> SearchInfo<Board<'a>> for BoardSearchWithGoal<'a>{

    fn is_goal(&self, board: &Board<'a> ) -> bool {
        board.pieces == self.goal.pieces
    }

    fn expand_state(&self, board: &Board<'a>) -> Vec<Board<'a>> {
        board.children_filtered()
    }
    
    fn max_depth(&self) -> Option<u64>{
        self.max_depth
    }
}

#[derive(Debug)]
pub struct BoardSearchLastRow{
    pub piece_index: usize,
    pub max_depth: Option<u64>
}

impl <'a> SearchInfo<Board<'a>> for BoardSearchLastRow{

    fn is_goal(&self, board: &Board<'a> ) -> bool {
        for x in 0..3{
            if board.pieces[x][2] != self.piece_index{
                return false;
            }
        }
        true
    }
    

    fn expand_state(&self, board: &Board<'a>) -> Vec<Board<'a>> {
        board.children_filtered()
    }
    
    fn max_depth(&self) -> Option<u64>{
        self.max_depth
    }
}


#[derive(Debug)]
pub struct BoardSearchSomeChanges<'a>{
    pub goal: Board<'a>,
    pub max_depth: Option<u64>,
    pub changes: u8
}

impl <'a> SearchInfo<Board<'a>> for BoardSearchSomeChanges<'a>{

    fn is_goal(&self, board: &Board<'a> ) -> bool {
        if board.empty_coords() != self.goal.empty_coords(){
            return false
        }
        let mut changes = 0;
        
        for x in 0..3{
            for y in 0..3{
                if board.pieces[x][y] != self.goal.pieces[x][y]{
                    changes += 1;
                }
            }
        }
        changes == self.changes
    }

    fn expand_state(&self, board: &Board<'a>) -> Vec<Board<'a>> {
        board.children_filtered()
    }
    
    fn max_depth(&self) -> Option<u64>{
        self.max_depth
    }
}


#[derive(Debug)]
pub struct BoardSearchMoveOnlyTwoRows<'a>{
    pub goal: Board<'a>,
    pub max_depth: Option<u64>,
}

impl <'a> SearchInfo<Board<'a>> for BoardSearchMoveOnlyTwoRows<'a>{

    fn is_goal(&self, board: &Board<'a> ) -> bool {
        board.pieces == self.goal.pieces
    }

    fn expand_state(&self, board: &Board<'a>) -> Vec<Board<'a>> {
        board.children().iter().
            map(|p| p.0 ).
            filter(|c| c.is_some()).
            map(|c| c.unwrap()).
            filter(|b| b.piece(0,2) == board.piece(0,2) && b.piece(1,2) == board.piece(1,2) && b.piece(2,2) == board.piece(2,2)).
            collect()
    }
    
    fn max_depth(&self) -> Option<u64>{
        self.max_depth
    }
}



pub fn scrambled_board<'a>(initial_board: &Board<'a>, steps: usize ) -> Board<'a>{
    use rand::rngs::StdRng;
    use rand::SeedableRng;
    use rand::Rng;
    
    let mut board = initial_board.clone();
    let mut rng = StdRng::seed_from_u64(1);
    
    for _ in 0..steps{
        let children = board.children_filtered();
        let index = rng.gen_range(0,children.len());
        board = children[index];
    }
    board
}


#[cfg(test)]
mod tests {

    use crate::crossteaser::crossteaser_search::*;
    use crate::search::astar::*;



    #[test]
    fn is_goal(){
        let piece_set = PieceSet::from_piece(&Piece::seed());
        let search = BoardSearchAnyColor{};

        for i in 0.. piece_set.pieces.len(){
            let board = Board::from_initial(&piece_set,i);
            assert!(search.is_goal(&board));
        }
    }

    
    #[test]
    fn root_is_goal() {
        let piece_set = PieceSet::from_piece(&Piece::seed());
        let board = Board::from_initial(&piece_set,0);
        let search = BoardSearchAnyColor{};
        let (found,_,_) = a_star_search(board,&search);
        assert!(found.is_some());
        assert!(found.unwrap().borrow().state == board);
    }

    #[test]
    fn search_on_scrambled_board() {

        
        
        fn search_with_step(step: usize){
            let piece_set = PieceSet::from_piece(&Piece::seed());
            let board = Board::from_initial(&piece_set,piece_set.get_piece_index_of_initial_piece());
            let scrambled = scrambled_board(&board,step);

            println!("Probando con paso:{} -- board:{} -- scrambled:{}", step, board, scrambled);
            
            let search = BoardSearchAnyColor{};
            let (found,_,_) = a_star_search(scrambled,&search);
            let found = found.unwrap();
            assert!(found.borrow().state == board);

            let to_root = root_path_state(&found);
            //to_root.iter().for_each( |b| println!("{}",b) );
            to_root.iter().for_each( |b| println!("{}\n\n", b.ascii_art_string()) );

            assert!(to_root[0] == scrambled);
            assert!(to_root[to_root.len()-1] == board);
        }

        let max = 50;
        
        for step in 1..max{
            search_with_step(step);
        }
        
    }
}
