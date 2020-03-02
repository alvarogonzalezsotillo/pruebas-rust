
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
struct PieceSet<'a>{
    pieces : Vec<Piece>,
    rotations : Vec<[usize;4]>,
    rotable_pieces_cache : OnceCell<Vec<RotablePiece<'a>>>,
    rotable_pieces_ref_cache : OnceCell<Vec<&'a RotablePiece<'a>>>,
}


#[derive(Debug)]
struct RotablePiece<'a>{
    piece_set : &'a PieceSet<'a>,
    piece_index : usize,
    rotations : [usize;4],
    rotations_ref : std::cell::UnsafeCell<Vec<&'a RotablePiece<'a>>>,
    
}

impl <'a> RotablePiece<'a>{
    pub fn rotate(&self, direction: Direction) -> &RotablePiece<'a> {
        let index = self.rotations[direction as usize];
        &self.piece_set.rotable_pieces()[index]
    }

    // pub fn rotate2(&mut self, direction: Direction) -> &RotablePiece<'a> {
    //     if self.rotations_ref.into_inner().len() == 0 {
    //         let ret = self.rotations.iter().map(|i|{
    //             self.piece_set.rotable_pieces()[*i]
    //         }).collect();

    //         *self.rotations_ref.get() = ret;
    //     }

    //     self.rotations_ref.into_inner()[direction as usize]
    // }

    pub fn piece(&self) -> &Piece{
        &self.piece_set.pieces[self.piece_index]
    }
}

impl <'a> PieceSet<'a>{

    pub fn rotable_pieces(&'a self) -> &Vec<&RotablePiece> {
        self.rotable_pieces_cache.get_or_init( || self.compute_rotable_pieces() );
        self.rotable_pieces_ref_cache.get_or_init( ||{
           self.rotable_pieces_cache.get().unwrap().iter().map(|p : &RotablePiece | p ).collect()
        })
    }


    fn compute_rotable_pieces(&'a self) -> Vec<RotablePiece> {
        (0 .. self.pieces.len()).map(|i|{
            RotablePiece{
                piece_set: self,
                piece_index: i,
                rotations : self.rotations[i],
                rotations_ref : std::cell::UnsafeCell::new(Vec::new())
            }
        }).collect()
    }
    
    fn compute_rotations(pieces: &Vec<Piece>) -> Vec<[usize;4]> {
        let mut ret = Vec::with_capacity(pieces.len());
        for _ in 0 .. pieces.len() {
            ret.push( [0,0,0,0] );
        }
        
        for direction in 0..4{
            let direction = Direction::from_u8(direction).unwrap();
            for piece in pieces.iter(){
                let index = Self::index_of(piece,pieces).unwrap();
                let index_of_rotation = Self::index_of(&piece.rotate(direction),pieces).unwrap();
                ret[index][direction as usize] = index_of_rotation;
            }
        }

        ret
    }

    fn index_of(piece: &Piece, vec: &Vec<Piece>) -> Option<usize> {
        vec.iter().position(|p| *p == *piece )
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
                if Self::index_of(p,&ret).is_none(){
                    println!("still not processed:{:?}", p);
                    ret.push(*p);
                    if Self::index_of(p,&not_processed_pieces).is_none(){
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
            rotable_pieces_cache: OnceCell::new(),
            rotable_pieces_ref_cache: OnceCell::new(),
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
    fn create_rotable_pieces(){
        let piece_set = PieceSet::from_piece(&Piece{colors:[C1,C2,C3,C4,C5,C6]});
        let rotable_pieces = piece_set.rotable_pieces();

        let rp1 = rotable_pieces[0];
        let rp2 = rp1.rotate(East);
        let rp3 = rp2.rotate(South);

        let p1 = piece_set.pieces[0];
        let p2 = p1.rotate(East);
        let p3 = p2.rotate(South);

        assert!( p1 == *rp1.piece());
        assert!( p2 == *rp2.piece());
        assert!( p3 == *rp3.piece());
    }

    #[test]
    fn rotable_faster_than_regular_pieces(){

        fn measure_time<T>( msg: &str, function : &dyn Fn()  -> T ) -> (u128,T) {
            let now = std::time::Instant::now();
            let ret = function();
            let millis = now.elapsed().as_millis();
            println!("{}: {}",msg, millis);
            (millis,ret)
        };
        
        let piece_set = PieceSet::from_piece(&Piece{colors:[C1,C2,C3,C4,C5,C6]});
        let rotable_pieces = piece_set.rotable_pieces();
        let times = 1000000;

        let (millis_regular,final_regular_piece) = measure_time("regular", &||{
            let mut piece = piece_set.pieces[0];
            for i in 0..times{
                piece = piece.rotate(North);
            }
            piece
        });

        let (millis_rotable,final_rotable_piece) = measure_time("rotable", &||{
            let mut piece = rotable_pieces[0];
            for i in 0..times{
                piece = piece.rotate(North);
            }
            piece
        });

        assert!( final_regular_piece == *final_rotable_piece.piece() );
        assert!( millis_regular > millis_rotable );
           
        
    }
    

    
}
