// https://www.quantamagazine.org/the-map-of-mathematics-20200213/
use std::env;

pub mod crossteaser;
pub mod ravioli;
pub mod search;




fn main() {
    let args : Vec<String> = env::args().collect();
    println!("Los argumentos son: {:?}", args);

    fn search_on_scrambled_board() {

        use crate::crossteaser::crossteaser_search::*;
        use crate::search::astar::*;
        use crate::crossteaser::*;
        
        use crate::search::*;
        
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

    search_on_scrambled_board()

}




