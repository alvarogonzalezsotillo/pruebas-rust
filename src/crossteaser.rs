
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

impl Color{
    pub fn letter(&self) -> char {
        match self {
            Color::C1 => '1',
            Color::C2 => '2',
            Color::C3 => '3',
            Color::C4 => '4',
            Color::C5 => '5',
            Color::C6 => '6',
        }
    }
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

    pub fn traslate(&self, coords: (i8,i8) ) -> (i8,i8){
        use Direction::*;
        match self{
            North => (coords.0,  coords.1-1),
            East =>  (coords.0+1,coords.1),
            South => (coords.0,  coords.1+1),
            West =>  (coords.0-1,coords.1),
            _ => panic!("No se puede rotar")
        }
    }

    pub fn opposite(&self) -> Direction{
        use Direction::*;
        match self{
            North => South,
            East =>  West,
            South => North,
            West =>  East,
            Up => Down,
            Down => Up
        }
    }
}


#[derive(Debug,Eq,PartialEq,Hash,Clone,Copy)]
struct Piece{
    colors : [Color;6],
}


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

            let next_piece = not_processed_pieces.pop().unwrap();
            let rotations : [Piece;4] = next_piece.rotations();

            rotations.to_vec().iter().for_each( |p|{
                if Self::index_of_piece(p,&ret).is_none(){
                    ret.push(*p);
                    if Self::index_of_piece(p,&not_processed_pieces).is_none(){
                        not_processed_pieces.push(*p);
                    }
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

    pub fn seed() -> Piece {
        use Color::*;
        Piece{colors:[C1,C2,C3,C4,C5,C6]}
    }

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

#[derive(Copy,Clone)]
struct Board<'a>{
    piece_set : &'a PieceSet,
    pieces : [[usize;3];3],
}

impl PartialEq for Board<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.pieces == other.pieces
    }
}

impl <'a> Board<'a>{

    pub fn coords_to_i8( coords: (usize,usize) ) -> (i8,i8) {
        (coords.0 as i8, coords.1 as i8)
    }

    pub fn coords_to_usize( coords: (i8,i8) ) -> (usize,usize) {
        (coords.0 as usize, coords.1 as usize)
    }

    
    pub fn children(&self) -> Vec<Option<Board<'a>>> {
        let empty_coords = self.empty_coords();
        (0..3).map(|d|{
            let direction = Direction::from_u8(d).unwrap();
            let coords = direction.opposite().traslate( Self::coords_to_i8(empty_coords) );
            self.rotate( Self::coords_to_usize(coords), direction )
        }).collect()
    }

    pub fn ascii_art(&self) -> [[char;9];9] {

        let mut b = [[' ';9];9];

        for x in 0..3 {
            for y in 0..3 {
                let piece_index = self.pieces[x][y];
                if  piece_index == Board::empty() {
                    continue;
                }
                
                let piece = self.piece_set.pieces[piece_index];

                let o = (1+x*3,1+y*3);
                
                for d in 0..4 {
                    let direction = Direction::from_u8(d).unwrap();
                    let p = direction.traslate( (o.0 as i8, o.1 as i8) );
                    let color = piece.color(direction);

                    b[p.1 as usize][p.0 as usize] = color.letter();
                }
                
                let color = piece.color(Direction::Up);
                b[o.1 as usize][o.0 as usize] = color.letter();
                
                
            }
        }

        b
    }

    pub fn ascii_art_string(&self) -> String {
        let ascii_art = self.ascii_art();
        let strings : Vec<String>  = ascii_art.iter().map( |v| {
            v.into_iter().collect()
        }).collect();

        strings.join("\n")
    }
    
    pub fn one_piece(piece_set:&'a PieceSet, coords: (usize,usize), piece: usize ) -> Board<'a> {
        let mut pieces = [[Self::empty();3];3];
        pieces[coords.0][coords.1] = piece;
        Board{
            piece_set,
            pieces
        }
    }
    
    pub fn empty() -> usize {
        99
    }
    
    pub fn is_empty(&self, coords: (usize,usize) ) -> bool{
        self.pieces[coords.0][coords.1] == Self::empty()
    }
    
    pub fn empty_coords(&self) -> (usize,usize){
        for x in 0..4{
            for y in 0..4{
                if self.is_empty( (x,y) ){
                    return (x,y);
                }
            }
        }
        panic!("Sin vacio");
    }

    pub fn inside( &self, coords: (i8,i8) ) -> bool{
        coords.0 >= 0 &&
            coords.0 < self.pieces.len() as i8 &&
            coords.1 >= 0 &&
            coords.1 <= self.pieces[0].len() as i8
    }

    pub fn rotate(&self, coords: (usize,usize), d: Direction) -> Option<Board<'a>>{

        if self.is_empty(coords){
            return None;
        }
        
        let to = d.traslate((coords.0 as i8,coords.1 as i8));
        if !self.inside(to) {
            return None;
        }
        let to = (to.0 as usize, to.1 as usize);
        if !self.is_empty(to) {
            return None;
        }

        
        let old_piece = self.pieces[coords.0][coords.1];

        use Direction::*;

        
        let piece = match d{
            North | South => self.piece_set.rotate( old_piece, d as usize ),
            East | West => self.piece_set.rotate( old_piece, d.opposite() as usize ),
            _ => panic!("No se puede rotar así")
        };

        let mut pieces = self.pieces.clone();
        pieces[coords.0][coords.1] = Self::empty();
        pieces[to.0][to.1] = piece;
        
        Some(
            Board{
                piece_set: self.piece_set,
                pieces
            }
        )
    }
}

#[cfg(test)]
mod tests {

    use crate::crossteaser::Direction;
    use crate::crossteaser::Color::*;
    use crate::crossteaser::Direction::*;
    use crate::crossteaser::Piece;
    use crate::crossteaser::PieceSet;
    use crate::crossteaser::Board;
    
    #[test]
    fn rotate() {

        
        let p1 = Piece::seed();
        let p2 = p1.rotate(South);
        let p3 = p2.rotate(North);

        println!("Pieza original: {:?}", p1 );
        println!("Pieza rotada: {:?}", p3);

        assert!(p1==p3);

        assert!(!std::ptr::eq(&p2,&p3));
    }

    #[test]
    fn create_pieceset(){
        let p1 = Piece::seed();
        let piece_set = PieceSet::from_piece(&p1);
        println!("pieceSet:{:?}", piece_set );
        assert!(piece_set.pieces.len() == 6*4);
    }

    #[test]
    fn create_pieces(){
        let piece_set = PieceSet::from_piece(&Piece::seed());
        
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
    fn index_rotations_same_as_regular_pieces(){
        use rand::prelude::*;
        
        let piece_set = PieceSet::from_piece(&Piece::seed());

        let mut index = 0;
        let mut piece = piece_set.pieces[index];

        for _ in 0..100 {
            let direction : u8 = random::<u8>()%4;
            index = piece_set.rotate(index, direction as usize );
            piece = piece.rotate(Direction::from_u8(direction).unwrap());
        }

        assert!( piece == piece_set.pieces[index] );
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
        
        let piece_set = PieceSet::from_piece(&Piece::seed());
        let times : usize= 1000000;

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
        assert!( millis_regular > millis_index );
    }
    

    #[test]
    fn one_piece_board_1(){
        
        let piece_set = PieceSet::from_piece(&Piece::seed());
        let piece_index = 0;
        
        let board1 = Board::one_piece(&piece_set, (0,0), piece_index);
        assert!( board1.pieces[0][0] == piece_index);
        assert!( board1.pieces[0][1] == Board::empty());
        
        let board2 = board1.rotate((0,0),South).unwrap();
        assert!( board2.pieces[0][0] == Board::empty());
        assert!( board2.pieces[0][1] == piece_set.rotate(piece_index,South as usize) );
    }


    #[test]
    fn one_piece_board_2(){
        
        let piece_set = PieceSet::from_piece(&Piece::seed());
        let piece_index = 0;
        
        let board1 = Board::one_piece(&piece_set, (0,0), piece_index);
        let board2 = board1.rotate((0,0),South).unwrap();

        let board3 = Board::one_piece(&piece_set, (0,1), piece_set.rotate(piece_index,South as usize) );

        println!( "BOARD1:\n{}", board1.ascii_art_string() );
        println!( "BOARD2:\n{}", board2.ascii_art_string() );
        
        assert!( board2 == board3 );
    }


    #[test]
    fn one_piece_board_3(){
        
        let piece_set = PieceSet::from_piece(&Piece::seed());
        let piece_index = 0;
        
        let mut board = Board::one_piece(&piece_set, (0,0), piece_index);
        let mut coords : (i8,i8) = (0,0);
        let directions = vec![South,South,East,East,North,North,West,West];

        println!("Original" );
        println!("{}", board.ascii_art_string());
        
        directions.iter().for_each( |d| {
            board = board.rotate( (coords.0 as usize, coords.1 as usize), *d ).unwrap();
            let new_coords = d.traslate(coords);
            coords = (new_coords.0 as i8, new_coords.1 as i8);
            println!("Tras {:?}", d );
            println!("{}", board.ascii_art_string());
        });

        assert!( board == Board::one_piece(&piece_set, (0,0), piece_index) );

    }

    
}
