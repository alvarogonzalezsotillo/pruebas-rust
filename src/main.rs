
use std::env;



fn main() {
    let args : Vec<String> = env::args().collect();
    println!("Los argumentos son: {:?}", args);
}

#[derive(Copy,Clone,Debug,PartialEq,Eq,Hash)]
#[repr(usize)]
enum Color{
    C1,
    C2,
    C3,
    C4,
    C5,
    C6
}

use num_derive::FromPrimitive;    
#[derive(Debug,FromPrimitive,Clone,Copy)]
enum Direction{
    North = 0,
    East = 1,
    South = 2,
    West = 3,
    Up = 4,
    Down = 5
}

impl Direction{
    pub fn from_u8(d: u8) -> Option<Direction>{
        num_traits::FromPrimitive::from_u8(d)
    }
}


#[derive(Debug,Eq,PartialEq,Hash,Clone,Copy)]
struct Piece{
    colors : [Color;6],
}


use once_cell::sync::OnceCell;
#[derive(Debug)]
struct PieceSet{
    pieces : Vec<Piece>,
    rotations : Vec<[usize;4]>,
}

impl PieceSet{
    
    fn compute_rotations(pieces: &Vec<Piece>) -> Vec<[usize;4]> {
        let mut ret = Vec::with_capacity(pieces.len());
        for _ in 0 .. pieces.len() {
            ret.push( [0,0,0,0] );
        }
        
        for direction in 0..4{
            let direction = Direction::from_u8(direction).unwrap();
            for piece in pieces.iter(){
                let index = Self::index_of_piece(piece,pieces).unwrap();
                let index_of_rotation : usize = Self::index_of_piece(&piece.rotate(direction),pieces).unwrap();
                ret[index][direction as usize] = index_of_rotation;
            }
        }

        ret
    }

    fn index_of_piece(piece: &Piece, vec: &Vec<Piece>) -> Option<usize> {
        vec.iter().position(|p| *p == *piece )
    }

    pub fn index_of(&self, piece: &Piece ) -> Option<usize> {
        Self::index_of_piece(piece,&self.pieces)
    }

    fn rotate(&self, index: usize, direction: usize ) -> usize{
        self.rotations[index][direction] 
    }
    
    fn compute_pieces_from(piece: &Piece) -> Vec<Piece> {

        let mut ret : Vec<Piece> = Vec::new();
        let mut not_processed_pieces : Vec<Piece> = Vec::new();
        
        not_processed_pieces.push(*piece);

        loop {

            if not_processed_pieces.len() == 0 {
                break;
            }
            println!("not_processed_pieces:{:?}", not_processed_pieces);

            let next_piece = not_processed_pieces.pop().unwrap();
            println!("next_piece:{:?}", next_piece);

            let rotations : [Piece;4] = next_piece.rotations();
            println!("rotations:{:?}", rotations);


            rotations.to_vec().iter().for_each( |p|{
                if Self::index_of_piece(p,&ret).is_none(){
                    println!("still not processed:{:?}", p);
                    ret.push(*p);
                    if Self::index_of_piece(p,&not_processed_pieces).is_none(){
                        println!("still not in not_processed_pieces:{:?}", p);
                        not_processed_pieces.push(*p);
                    }
                }
                else{
                    println!("already processed:{:?}", p);
                }
            });
        }

        ret
    }

    pub fn from_piece<'b>(piece: &'b Piece) -> PieceSet {
        let pieces = Self::compute_pieces_from(piece);
        let rotations = Self::compute_rotations(&pieces);
        PieceSet{
            pieces : pieces,
            rotations : rotations,
        }
    }
}





impl Piece{
    pub fn color(&self, d: Direction) -> Color {
        self.colors[d as usize]
    }

    fn rotations(&self) -> [Piece;4]{
        use Direction::*;
        [
            self.rotate(North),
            self.rotate(East),
            self.rotate(South),        
            self.rotate(West),
        ]
    }




    pub fn rotate(&self, d: Direction ) -> Piece{
        use Direction::*;
        
        match d{
            North => Piece {
                colors: [
                    self.color(Up),
                    self.color(East),
                    self.color(Down),
                    self.color(West),
                    self.color(South),
                    self.color(North),
                ]
            },
            East => Piece {
                colors: [
                    self.color(North),
                    self.color(Up),
                    self.color(South),
                    self.color(Down),
                    self.color(West),
                    self.color(East),
                ]
            },
            South => Piece {
                colors: [
                    self.color(Down),
                    self.color(East),
                    self.color(Up),
                    self.color(West),
                    self.color(North),
                    self.color(South)
                ]
            },
            West => Piece {
                colors: [
                    self.color(North),
                    self.color(Down),
                    self.color(South),
                    self.color(Up),
                    self.color(East),
                    self.color(West),
                ]
            },
            
            _ => panic!("No se puede girar {:?}", d),           
        }

    }
}




#[cfg(test)]
mod tests {

    use crate::Direction;
    use crate::Color::*;
    use crate::Direction::*;
    use crate::Piece;
    use crate::PieceSet;
    
    #[test]
    fn rotate() {

        
        let p1 = Piece{colors:[C1,C2,C3,C4,C5,C6]};
        let p2 = p1.rotate(South);
        let p3 = p2.rotate(North);

        println!("Pieza original: {:?}", p1 );
        println!("Pieza rotada: {:?}", p3);

        assert!(p1==p3);

        assert!(!std::ptr::eq(&p2,&p3));
    }

    #[test]
    fn create_pieceset(){
        let p1 = Piece{colors:[C1,C2,C3,C4,C5,C6]};
        let piece_set = PieceSet::from_piece(&p1);
        println!("pieceSet:{:?}", piece_set );
        assert!(piece_set.pieces.len() == 6*4);
    }

    #[test]
    fn create_pieces(){
        let piece_set = PieceSet::from_piece(&Piece{colors:[C1,C2,C3,C4,C5,C6]});


        
        let i1 = 0;
        let i2 = piece_set.rotate(i1,East as usize);
        let i3 = piece_set.rotate(i2,South as usize);

        let p1 = piece_set.pieces[i1];
        let p2 = p1.rotate(East);
        let p3 = p2.rotate(South);

        assert!( piece_set.pieces[i1] == p1);
        assert!( piece_set.pieces[i2] == p2);
        assert!( piece_set.pieces[i3] == p3);
    }

    #[test]
    fn index_faster_than_regular_pieces(){

        fn measure_time<T>( msg: &str, function : &dyn Fn()  -> T ) -> (u128,T) {
            let now = std::time::Instant::now();
            let ret = function();
            let millis = now.elapsed().as_millis();
            println!("{}: {}",msg, millis);
            (millis,ret)
        };
        
        let piece_set = PieceSet::from_piece(&Piece{colors:[C1,C2,C3,C4,C5,C6]});
        let times : usize= 10000000;

        let (millis_regular,final_regular_piece) = measure_time("regular", &||{
            let mut piece = piece_set.pieces[0];
            for i in 0..times{
                let direction = Direction::from_u8((i%4) as u8).unwrap();
                piece = piece.rotate(direction);
            }
            piece
        });

        let (millis_index,final_index) = measure_time("index", &||{
            let mut piece = 0;
            for i in 0..times{
                piece = piece_set.rotate(piece,i%4);
            }
            piece
        });

        assert!( final_regular_piece == piece_set.pieces[final_index] );
        assert!( millis_regular < millis_index );
           
        
    }
    

    
}
