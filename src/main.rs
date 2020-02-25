
use std::env;


struct MyStruct {
    x: i32,
    y: i32,
    seed : Piece,
    pub pieces: Vec<Piece>,
    rotations : Vec<Vec<usize>>

}

impl MyStruct {
    #[inline]
    pub fn my_static() -> i32 {
        123
    }
}

fn main() {
    let args : Vec<String> = env::args().collect();
    println!("Los argumentos son: {:?}", args);

    struct SomeStruct{
        field: String
    }

    impl SomeStruct{
        fn new(field:String) -> SomeStruct{
            println!("The constructor is expensive, should be avoided if possible.");
            SomeStruct{field}    
        }
    }

    fn create_vec_in_loop_with_map(times: usize) -> Vec<Vec<SomeStruct>> {
        let mut ret = Vec::new();
        for _i in 0 .. times{
            let array = [1,2,3];
            let item = array.iter().map( |x| SomeStruct{ field: x.to_string() } ).collect();
            ret.push(item);
        }

        ret
    }

    
    fn create_vec(len: usize) -> Vec<SomeStruct> {
        let mut ret = Vec::new();
        for i in 0.. len{
            ret.push( SomeStruct{ field: i.to_string() } );
        }
        ret
    }

    fn create_vec_in_loop(times: usize) -> Vec<Vec<SomeStruct>> {
        let mut ret = Vec::new();
        for i in 0 .. times{
            let mut item = Vec::new();
            item.push( SomeStruct{ field: i.to_string() } );
            ret.push(item);
        }

        ret
    }

    println!("hola");
    
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

#[derive(Debug)]
enum Direction{
    North = 0,
    East = 1,
    South = 2,
    West = 3,
    Up = 4,
    Down = 5
}


#[derive(Debug,Eq,PartialEq,Hash,Clone,Copy)]
struct Piece{
    colors : [Color;6],
}


#[derive(Debug)]
struct PieceSet{
    pieces_to_rotations : std::collections::HashMap<Piece,[Piece;4]>,
}

impl PieceSet{

    
    pub fn from_piece(piece: Piece) -> PieceSet {


        let mut pieces_to_rotations : std::collections::HashMap<Piece,[Piece;4]> = std::collections::HashMap::new();
        let mut not_processed_pieces : Vec<Piece> = Vec::new();
        
        not_processed_pieces.push(piece);

        loop {

            if not_processed_pieces.len() == 0 {
                break;
            }
            println!("not_processed_pieces:{:?}", not_processed_pieces);

            let next_piece = not_processed_pieces.pop().unwrap();
            println!("next_piece:{:?}", next_piece);

            let rotations : &[Piece;4] = pieces_to_rotations.entry(next_piece).or_insert_with( || next_piece.rotations() );
            println!("rotations:{:?}", rotations);


            rotations.to_vec().iter().for_each( |p|{
                if !pieces_to_rotations.contains_key(p){
                    println!("still not processed:{:?}", p);
                    
                    not_processed_pieces.push(*p);
                }
                else{
                    println!("already processed:{:?}", p);
                }
            });
        }

        PieceSet{
            pieces_to_rotations,
        }
    }

    pub fn pieces(&self) -> Vec<&Piece>{
        self.pieces_to_rotations.keys().collect()
    }

    pub fn rotate(&self, piece:&Piece, direction: Direction) -> Piece {
        self.pieces_to_rotations.get(piece).unwrap()[direction as usize]
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
        let piece_set = PieceSet::from_piece(p1);
        println!("pieceSet:{:?}", piece_set );
        assert!(piece_set.pieces().len() == 6*4);
    }

    #[test]
    fn pieceset_rotate(){
        let p1 = Piece{colors:[C1,C2,C3,C4,C5,C6]};
        let piece_set = PieceSet::from_piece(p1);

        assert!(p1.rotate(North) == piece_set.rotate(&p1,North) );
        assert!(p1.rotate(North).rotate(East) == piece_set.rotate( &piece_set.rotate(&p1,North), East ) );
    }

    
}
