// https://www.quantamagazine.org/the-map-of-mathematics-20200213/
use std::env;

pub mod crossteaser;
pub mod ravioli;
pub mod search;

use crate::crossteaser::crossteaser_search::*;
use crate::search::astar::*;
use crate::search::*;


fn estados_posibles() {
    let posiciones_de_una_pieza = 6.0 * 4.0;
    let mut piezas_en_8_sitios = 1.0;
    for _x in 1..8 {
        piezas_en_8_sitios = piezas_en_8_sitios * posiciones_de_una_pieza
    }
    println!("Estados posibles: {}", piezas_en_8_sitios)
}

fn resuelve_016<'a>(goal: Board<'a>, board: Board<'a>, max_level: u64 ) -> bool {

    
    let search_exact_changes = BoardSearchExactChanges {
        goal: goal,
        max_depth: Some(max_level),
        changes: vec!(0,1,6,7)
    };
    let (found, _, _) = a_star_search(board, &search_exact_changes);
    if found.is_none(){
        println!( "Sin aproximación a 016");
        return false;
    }
    let board_up_to_016 = found.unwrap().borrow().state;

    let changes = vec!(
        //vec!(0,1,6,7),
        //vec!(0,1,6),
        //vec!(0,1,7),
        //vec!(0,6,7),
        vec!(0,1),
        //vec!(0,6),
        //vec!(0,7),
        //vec!(1,6),
        //vec!(1,7),
        //vec!(6,7),
    );

    let moves : Vec<Option<(Vec<usize>,Vec<Direction>)>> = changes.iter().map( |changes| {
        println!( "Buscando movimientos para:{:?}", changes);
        moves_for_changes(changes.clone(), max_level).map(|moves| (changes.clone(),moves) )
    }).collect();

    let moves : Vec<Vec<Direction>> = moves.into_iter().filter( |o| o.is_some() ).map( |o| o.unwrap().1 ).collect();

    
    let search = BoardSearchCustomMoves{
        delegate: &BoardSearchWithGoal{
            goal: goal,
            max_depth: Some(max_level)
        },
        moves: moves
    };
    println!( "Aproximación a 016:\n{}", board_up_to_016.ascii_art_string() );
    println!("Tengo todas las aproximaciones finales");
    let (found, _, expanded_nodes) = a_star_search(board_up_to_016, &search);
    println!( "Nodos expandidos:{}", expanded_nodes.len() );
    match found{
        None => {
            false
        }
        Some(found) => {
            let to_root = root_path_state(&found);
            println!("SOLUCION ENCONTRADA:\n");
            to_root.iter().for_each( |b| println!("{}\n\n",b.ascii_art_string()) );
            true
        }
    }
}


fn aproxima<'a>(goal: Board<'a>, board: Board<'a>, changes: u8, max_level: u64 ) -> Option<(Vec<Direction>,Board<'a>)> {

    let search_some_changes = BoardSearchSomeChanges {
        goal: goal,
        max_depth: Some(max_level),
        changes: changes,
    };

    let (found, _, _) = a_star_search(board, &search_some_changes);
    match found{
        None => {
            None
        }
        Some(found) => {
            let to_root = root_path_state(&found);
            println!("APROXIMACION ENCONTRADA:\n");
            to_root.iter().for_each( |b| println!("{}\n\n",b.ascii_art_string()) );
            let moves = Board::infer_moves_to_empty_position(to_root);
            let ret_board = found.borrow().state.clone_with_pieceset(goal.piece_set);
            Some(
                (
                    moves,
                    ret_board
                )
            )
        }
    }
}

#[allow(dead_code)]
fn soluciona_por_pasos<'a>(goal: Board<'a>, board: Board<'a>) -> bool {
    println!("GOAL:");
    println!("{}\n\n\n\n", goal.ascii_art_string());

    println!("BOARD:");
    println!("{}\n\n\n\n", board.ascii_art_string());
    
    let max_level = 28;

    let aproximacion = aproxima(goal,board,3,max_level);
    if aproximacion.is_none(){
        println!("No hay aproximación a 3");
        return false;
    }
    let aproximacion = aproximacion.unwrap();
    println!("Aproximación a 3:{:?}", aproximacion.0 );
    println!("{}\n\n", aproximacion.1.ascii_art_string() );


    let aproximacion = aproxima(goal,aproximacion.1,2,max_level);
    if aproximacion.is_none(){
        println!("No hay aproximación a 2");
        return false;
    }
    let aproximacion = aproximacion.unwrap();
    println!("Aproximación a 2:{:?}", aproximacion.0 );
    println!("{}\n\n", aproximacion.1.ascii_art_string() );

    let diffs = goal.compute_difs(&aproximacion.1);
    let moves = moves_for_changes(diffs.clone(), max_level);
    if moves.is_none(){
        println!("No hay movimientos para diferencias finales: {:?}", diffs );
        return false;
    }

    let moves = moves.unwrap();
    let board_copy = aproximacion.1.clone();
    let mut current = board_copy.apply_moves_to_empty_position(&moves).
        unwrap().
        last().
        unwrap().clone();
    println!("Aplico rotación para diferencias finales: {:?}", moves );
    while current != board_copy{
        current = current.apply_moves_to_empty_position(&moves).
            unwrap().
            last().
            unwrap().clone();
        println!("Aplico rotación para diferencias finales: {:?}", moves );

        if current == goal{
            return true;
        }
    }
    
    println!("La rotación final no ha tenido éxito" );

    return false;
}



fn main() {
    let args: Vec<String> = env::args().collect();
    println!("Los argumentos son: {:?}", args);

    estados_posibles();

    #[allow(unused_imports)]
    use crate::crossteaser::crossteaser_search::Color::{B, G, O, P, R, Y};

    let piece_set = PieceSet::from_piece(&Piece::seed());

    for piece_index in 1..piece_set.get_number_of_pieces() {
        println!("Probando con pieza número:{}", piece_index );
        let goal = Board::from_piece(&piece_set, piece_index);

        let colors_original: [Option<[Color; 2]>; 9] = [
            Some([O, P]),
            Some([R, O]),
            Some([O, B]),
            Some([B, G]),
            None,
            Some([B, P]),
            Some([O, R]),
            Some([Y, B]),
            Some([Y, R]),
        ];
        let original = Board::from_colors(&piece_set, colors_original);
        if resuelve_016(goal, original, 26 ) {
            println!("ALBRICIAS!!!!");
            return;
        }
        return;
    }
}
