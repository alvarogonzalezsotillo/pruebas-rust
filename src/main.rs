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


fn soluciona_por_pasos<'a>(goal: Board<'a>, board: Board<'a> ) -> bool{

    println!("GOAL:");
    println!("{}\n\n\n\n", goal.ascii_art_string());

    let max_level = 28;
    
    let search_last_row = BoardSearchLastRow{
        piece_index: goal.piece(0,0),
        max_depth: Some(max_level)
    };
    let (found,_,_) = a_star_search(board,&search_last_row);
    if !found.is_some() {
        println!( "NO HUBO SUERTE1");
        return false;
    }
    let found = found.unwrap();
    let to_root = root_path_state(&found);
    to_root.iter().for_each( |b| println!("{}\n\n", b.ascii_art_string()) );
    println!( "1******************************************************");
    
    let search_some_changes = BoardSearchSomeChanges{
        goal: goal,
        max_depth: Some(max_level),
        changes: 2
    };
    let (found,_,_) = a_star_search(found.borrow().state,&search_some_changes);
    if !found.is_some() {
        println!( "NO HUBO SUERTE2");
        return false;
    }
    let found = found.unwrap();
    let to_root = root_path_state(&found);
    to_root.iter().for_each( |b| println!("{}\n\n", b.ascii_art_string()) );
    println!( "2******************************************************");

    let search_some_changes = BoardSearchSomeChanges{
        goal: goal,
        max_depth: Some(max_level),
        changes: 0
    };
    let (found,_,_) = a_star_search(found.borrow().state,&search_some_changes);
    if !found.is_some() {
        println!( "NO HUBO SUERTE3");
        return false;
    }
    let found = found.unwrap();
    let to_root = root_path_state(&found);
    to_root.iter().for_each( |b| println!("{}\n\n", b.ascii_art_string()) );
    println!( "3******************************************************");

    return true;
}


fn soluciona_por_niveles<'a>(goal: Board<'a>, board: Board<'a> ) -> bool{

    println!("GOAL:");
    println!("{}\n\n\n\n", goal.ascii_art_string());

    let max_level = 28;
    
    let search_last_row = BoardSearchLastRow{
        piece_index: goal.piece(0,0),
        max_depth: Some(max_level)
    };
    let (found,_,_) = a_star_search(board,&search_last_row);
    if !found.is_some() {
        println!( "NO HUBO SUERTE1");
        return false;
    }
    let found = found.unwrap();
    let to_root = root_path_state(&found);
    to_root.iter().for_each( |b| println!("{}\n\n", b.ascii_art_string()) );
    println!( "1******************************************************");
    
    let search_two_rows = BoardSearchMoveOnlyTwoRows{
        goal: goal,
        max_depth: Some(68)
    };
    let (found,_,_) = a_star_search(found.borrow().state,&search_two_rows);
    if !found.is_some() {
        println!( "NO HUBO SUERTE2");
        return false;
    }
    let found = found.unwrap();
    let to_root = root_path_state(&found);
    to_root.iter().for_each( |b| println!("{}\n\n", b.ascii_art_string()) );
    println!( "2******************************************************");

    return true;
}


fn main() {
    let args : Vec<String> = env::args().collect();
    println!("Los argumentos son: {:?}", args);

    estados_posibles();


    #[allow(unused_imports)]
    use crate::crossteaser::crossteaser_search::Color::{G,R,P,B,Y,O};
    
    let piece_set = PieceSet::from_piece(&Piece::seed());

    for piece_index in 0 .. piece_set.get_number_of_pieces() {
        
        let goal = Board::from_piece(&piece_set, piece_index);
        
        let colors_original : [Option<[Color;2]>;9] = [
            Some([O, P]), Some([R, O]), Some([O, B]),
            Some([B, G]), None,         Some([B, P]),
            Some([O, R]), Some([Y, B]), Some([Y, R])
        ];
        let original = Board::from_colors(&piece_set, colors_original);

        //soluciona_por_pasos(goal,original);
        if soluciona_por_niveles(goal,original) {
            println!( "ALBRICIAS!!!!");
            return;
        }
    }
}




