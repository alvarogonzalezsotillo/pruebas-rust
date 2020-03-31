
use crate::crossteaser::*;
use crate::crossteaser::Direction::*;
use crate::search::*;
use crate::ravioli::*;


impl <'a> std::hash::Hash for Board<'a>{
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.pieces.hash(state);
    }    
}

impl <'a> State for Board<'a>{
}

#[derive(Debug)]
struct BoardSearch{
}

impl <'a> SearchInfo<Board<'a>> for BoardSearch{
    fn is_goal(&self, board: &Board<'a>) -> bool{
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
        board.children().iter().
            filter( |o| o.is_some() ).
            map( |o| o.unwrap() ).
            collect()
    }

    fn heuristic(&self,_state: &Board<'a>) -> u64{
        0
    }
    
}


#[cfg(test)]
mod tests {

    use crate::crossteaser::crossteaser_search::*;
    use crate::crossteaser::*;
    use crate::search::*;
    use crate::ravioli::*;
    use crate::search::astar::*;
    use rand::Rng;


    fn scrambled_board<'a>(piece_set: &'a PieceSet, piece_index: usize, steps: usize ) -> Board<'a>{
        let mut board = Board::from_initial(piece_set,piece_index);
        let mut rng = rand::thread_rng();
        
        for _ in 0..steps{
            let children = board.children_filtered();
            let index = rng.gen_range(0,children.len());
            board = children[index];
        }
        board
    }

    #[test]
    fn is_goal(){
        let piece_set = PieceSet::from_piece(&Piece::seed());
        let search = BoardSearch{};

        for i in 0.. piece_set.pieces.len(){
            let board = Board::from_initial(&piece_set,i);
            assert!(search.is_goal(&board));
        }
    }

    
    #[test]
    fn root_is_goal() {
        let piece_set = PieceSet::from_piece(&Piece::seed());
        let board = Board::from_initial(&piece_set,0);
        let search = BoardSearch{};
        let found = a_star_search(board,&search);
        assert!(found.is_some());
        assert!(found.unwrap().borrow().state == board);
    }

    #[test]
    fn search_on_scrambled_board() {

        fn search_with_step(step: usize){
            let piece_set = PieceSet::from_piece(&Piece::seed());
            let board = Board::from_initial(&piece_set,0);
            let scrambled = scrambled_board(&piece_set,0,step);

            println!("Probando con paso:{} -- {}", step, scrambled);
            
            let search = BoardSearch{};
            let found = a_star_search(scrambled,&search);
            assert!(found.is_some());
            let found = found.unwrap();
            assert!(found.borrow().state == board);

            let to_root = root_path_state(&found);
            to_root.iter().for_each( |b| println!("{}",b) );
            assert!(to_root[to_root.len()-1] == scrambled);
            assert!(to_root[0] == board);
        }

        for step in 1..50{
            search_with_step(step);
        }
        
    }

    
    


}
