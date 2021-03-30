// https://www.quantamagazine.org/the-map-of-mathematics-20200213/
use std::env;

pub mod crossteaser;
pub mod ravioli;
pub mod search;


fn estados_posibles(){
    let posiciones_de_una_pieza = 6.0*4.0;
    let  mut piezas_en_8_sitios = 1.0;
    for _x in 1..8{
        piezas_en_8_sitios = piezas_en_8_sitios * posiciones_de_una_pieza
    }
    println!("Estados posibles: {}", piezas_en_8_sitios)

}


fn main() {
    let args : Vec<String> = env::args().collect();
    println!("Los argumentos son: {:?}", args);

    estados_posibles();

    use crate::crossteaser::crossteaser_search::*;
    use crate::search::astar::*;
    use crate::search::*;
    use crate::crossteaser::crossteaser_search::Color::{G,R,P,B,Y,O};
        
    let piece_set = PieceSet::from_piece(&Piece::seed());
    let colors : [Option<[Color;2]>;9] = [
        Some([Y, O]), Some([G, B]), Some([R, P]),
        Some([O, B]), Some([B, O]), Some([P, O]),
        None, Some([R, Y]), Some([O, R])
    ];
        
    let board = Board::from_colors(&piece_set, colors);

    let search = BoardSearch{};
    let (found,_,_) = a_star_search(board,&search);
    assert!(found.is_some());
    let found = found.unwrap();

    let to_root = root_path_state(&found);
    to_root.iter().for_each( |b| println!("{}",b) );
    assert!(to_root[to_root.len()-1] == board);

}




