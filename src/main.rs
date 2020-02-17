
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

#[derive(Debug)]
enum Direction{
    North = 0,
    East = 1,
    South = 2,
    West = 3,
    Up = 4,
    Down = 5
}


#[derive(Debug,Eq,PartialEq,Hash,Clone)]
struct Piece{
    colors : [Color;6],
}

struct RotablePiece<'a>{
    pub piece: &'a Piece,
    rotations :  [&'a Piece;4]
}

struct PieceSet{
    pieces: Vec<Piece>,
}

impl PieceSet{

    pub fn from(p:Piece) -> PieceSet {
        use std::collections::HashSet;
        let mut allPieces : HashSet<Piece> = HashSet::new();
        let mut notProcessedPieces : HashSet<Piece> = HashSet::new();

        notProcessedPieces.insert(p);

        panic!("TBD");
        
        
    }
}





impl Piece{
    pub fn color(&self, d: Direction) -> Color {
        self.colors[d as usize]
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
    use crate::RotablePiece;
    
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

 
            
}
