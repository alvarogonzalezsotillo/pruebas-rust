// https://www.quantamagazine.org/the-map-of-mathematics-20200213/
use std::env;

pub mod crossteaser;
pub mod ravioli;
pub mod search;

use crate::crossteaser::crossteaser_search::*;
use crate::search::astar::*;
use crate::search::*;

fn estados_posibles(){
    let posiciones_de_una_pieza = 6.0*4.0;
    let  mut piezas_en_8_sitios = 1.0;
    for _x in 1..8{
        piezas_en_8_sitios = piezas_en_8_sitios * posiciones_de_una_pieza
    }
    println!("Estados posibles: {}", piezas_en_8_sitios)
}


fn soluciona_por_pasos<'a>(goal: Board<'a>, board: Board<'a> ){

    let piece_set = board.piece_set;
    let search_last_row = BoardSearchLastRow{
        piece_index: piece_set.get_piece_index_of_initial_piece(),
        max_depth: Some(25)
    };
    let (found,_,_) = a_star_search(board,&search_last_row);
    let found = found.unwrap();
    let to_root = root_path_state(&found);
    to_root.iter().for_each( |b| println!("{}\n\n", b.ascii_art_string()) );

    println!( "******************************************************");
    
    let search_some_changes = BoardSearchSomeChanges{
        goal: goal,
        max_depth: Some(29),
        changes: 2
    };
    let (found,_,_) = a_star_search(found.borrow().state,&search_some_changes);
    let found = found.unwrap();
    let to_root = root_path_state(&found);
    to_root.iter().for_each( |b| println!("{}\n\n", b.ascii_art_string()) );

    let search_some_changes = BoardSearchSomeChanges{
        goal: goal,
        max_depth: Some(29),
        changes: 1
    };
    let (found,_,_) = a_star_search(found.borrow().state,&search_some_changes);
    let found = found.unwrap();
    let to_root = root_path_state(&found);
    to_root.iter().for_each( |b| println!("{}\n\n", b.ascii_art_string()) );

}


fn main() {
    let args : Vec<String> = env::args().collect();
    println!("Los argumentos son: {:?}", args);

    estados_posibles();


    #[allow(unused_imports)]
    use crate::crossteaser::crossteaser_search::Color::{G,R,P,B,Y,O};
        
    let piece_set = PieceSet::from_piece(&Piece::seed());
    
    let colors_goal : [Option<[Color;2]>;9] = [
        Some([Y, R]), Some([Y, R]), Some([Y, R]),
        Some([Y, R]), None,         Some([Y, R]),
        Some([Y, R]), Some([Y, R]), Some([Y, R])
    ];
    let goal = Board::from_colors(&piece_set, colors_goal);
    
    let colors_original : [Option<[Color;2]>;9] = [
        Some([O, P]), Some([R, O]), Some([O, B]),
        Some([B, G]), None,         Some([B, P]),
        Some([O, R]), Some([Y, B]), Some([Y, R])
    ];
    let original = Board::from_colors(&piece_set, colors_original);

    soluciona_por_pasos(goal,original);
}




