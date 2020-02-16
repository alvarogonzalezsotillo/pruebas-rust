
use std::env;


fn main() {
    let args : Vec<String> = env::args().collect();
    println!("Los argumentos son: {:?}", args);

    use Color::*;
    
    let p1 = Piece{colors:[C1,C2,C3,C4,C5,C6]};
    let p2 = p1.rotate(Direction::South);

    println!("Pieza original: {:?}", p1 );
    println!("Pieza rotada: {:?}", p2);
}

#[derive(Copy,Clone,Debug)]
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


#[derive(Debug)]
struct Piece{
    colors : [Color;6],
}

struct RotablePiece<'a>{
    pub piece: &'a Piece,
    rotations : [&'a Piece;4]
}




impl<'a> Piece{
    pub fn color(&self, d: Direction) -> Color {
        self.colors[d as usize]
    }

    pub fn rotate(&self, d: Direction ) -> Piece{
        use Direction::*;
        
        match d{
 
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
            _ => panic!("No se puede girar {:?}", d),           
        }

    }
}

impl <'a> RotablePiece<'a>{
    pub fn rotate(&self, _d: Direction ) -> &RotablePiece {
        self
    }
}
