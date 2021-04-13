pub mod crossteaser_search;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[repr(usize)]
pub enum Color {
    //G,R,P,B,Y,O
    G,
    R,
    P,
    B,
    Y,
    O,
}

impl Color {
    pub fn letter(&self) -> char {
        match self {
            Color::G => 'G',
            Color::R => 'R',
            Color::P => 'P',
            Color::B => 'B',
            Color::Y => 'Y',
            Color::O => 'O',
        }
    }

    pub fn from_letter(letter: char) -> Option<Color> {
        match letter {
            '1' => Some(Color::G),
            '2' => Some(Color::R),
            '3' => Some(Color::P),
            '4' => Some(Color::B),
            '5' => Some(Color::Y),
            '6' => Some(Color::O),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    Up = 0,
    North = 1,
    East = 2,
    South = 3,
    West = 4,
    Down = 5,
}

impl Direction {
    pub fn posible_rotations() -> [Direction; 4] {
        use Direction::*;
        [North, East, South, West]
    }

    pub fn from_index(s: usize) -> Direction {
        use Direction::*;
        match s {
            0 => North,
            1 => East,
            2 => South,
            3 => West,
            _ => panic!("No es un índice válido"),
        }
    }

    pub fn to_index(&self) -> usize {
        use Direction::*;
        match self {
            North => 0,
            East => 1,
            South => 2,
            West => 3,
            Up => panic!("No es un índice"),
            Down => panic!("No es un índice"),
        }
    }

    pub fn traslate(&self, coords: (i8, i8)) -> (i8, i8) {
        use Direction::*;
        match self {
            North => (coords.0, coords.1 - 1),
            East => (coords.0 + 1, coords.1),
            South => (coords.0, coords.1 + 1),
            West => (coords.0 - 1, coords.1),
            _ => panic!("No se puede rotar"),
        }
    }

    pub fn opposite(&self) -> Direction {
        use Direction::*;
        match self {
            North => South,
            East => West,
            South => North,
            West => East,
            Up => Down,
            Down => Up,
        }
    }
}

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
pub struct Piece {
    pub colors: [Color; 6],
}

#[derive(Debug)]
pub struct PieceSet {
    pieces: Vec<Piece>,
    rotations: Vec<[usize; 4]>,
}

impl PieceSet {
    pub fn get_number_of_pieces(&self) -> usize {
        self.pieces.len()
    }

    pub fn get_piece_index_of_initial_piece(&self) -> usize {
        // EN LAS IMÁGENES DE INTERNET SALE RESUELTO CON AMARILLO Y ROJO
        self.get_piece_index_from_colors(Color::Y, Color::R)
            .unwrap()
    }

    pub fn get_piece_index_from_colors(
        &self,
        up_color: Color,
        north_color: Color,
    ) -> Option<usize> {
        use Direction::*;
        for i in 0..self.pieces.len() {
            let piece = &self.pieces[i];
            if piece.colors[Up as usize] == up_color && piece.colors[North as usize] == north_color
            {
                return Some(i);
            }
        }
        return None;
    }

    fn compute_rotations(pieces: &Vec<Piece>) -> Vec<[usize; 4]> {
        let mut ret = Vec::with_capacity(pieces.len());
        for _ in 0..pieces.len() {
            ret.push([0, 0, 0, 0]);
        }

        for direction in Direction::posible_rotations().iter() {
            for piece in pieces.iter() {
                let index = Self::index_of_piece(piece, pieces).unwrap();
                let index_of_rotation: usize =
                    Self::index_of_piece(&piece.rotate(*direction), pieces).unwrap();
                ret[index][Direction::to_index(direction)] = index_of_rotation;
            }
        }

        ret
    }

    fn index_of_piece(piece: &Piece, vec: &Vec<Piece>) -> Option<usize> {
        vec.iter().position(|p| *p == *piece)
    }

    pub fn index_of(&self, piece: &Piece) -> Option<usize> {
        Self::index_of_piece(piece, &self.pieces)
    }

    fn rotate(&self, index: usize, direction: Direction) -> usize {
        self.rotations[index][direction.to_index()]
    }

    fn compute_pieces_from(piece: &Piece) -> Vec<Piece> {
        let mut ret: Vec<Piece> = Vec::new();
        let mut not_processed_pieces: Vec<Piece> = Vec::new();

        not_processed_pieces.push(*piece);

        loop {
            if not_processed_pieces.len() == 0 {
                break;
            }

            let next_piece = not_processed_pieces.pop().unwrap();
            let rotations: [Piece; 4] = next_piece.rotations();

            rotations.to_vec().iter().for_each(|p| {
                if Self::index_of_piece(p, &ret).is_none() {
                    ret.push(*p);
                    if Self::index_of_piece(p, &not_processed_pieces).is_none() {
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
        PieceSet {
            pieces: pieces,
            rotations: rotations,
        }
    }
}

impl Piece {
    pub fn seed() -> Piece {
        use Color::*;
        Piece {
            colors: [G, R, P, B, Y, O],
        }
    }

    pub fn color(&self, d: Direction) -> Color {
        self.colors[d as usize]
    }

    fn rotations(&self) -> [Piece; 4] {
        use Direction::*;
        [
            self.rotate(North),
            self.rotate(East),
            self.rotate(South),
            self.rotate(West),
        ]
    }

    pub fn rotate(&self, d: Direction) -> Piece {
        use Direction::*;

        match d {
            North => Piece {
                colors: [
                    self.color(South),
                    self.color(Up),
                    self.color(East),
                    self.color(Down),
                    self.color(West),
                    self.color(North),
                ],
            },
            East => Piece {
                colors: [
                    self.color(West),
                    self.color(North),
                    self.color(Up),
                    self.color(South),
                    self.color(Down),
                    self.color(East),
                ],
            },
            South => Piece {
                colors: [
                    self.color(North),
                    self.color(Down),
                    self.color(East),
                    self.color(Up),
                    self.color(West),
                    self.color(South),
                ],
            },
            West => Piece {
                colors: [
                    self.color(East),
                    self.color(North),
                    self.color(Down),
                    self.color(South),
                    self.color(Up),
                    self.color(West),
                ],
            },

            _ => panic!("No se puede girar {:?}", d),
        }
    }
}

#[derive(Copy, Clone)]
pub struct Board<'a> {
    pub piece_set: &'a PieceSet,
    pieces: [[usize; 3]; 3],
}

impl std::fmt::Debug for Board<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({})", "Debug for Board")
    }
}

impl std::fmt::Display for Board<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut str: String = "".to_string();
        for x in 0..3 {
            for y in 0..3 {
                str = str + " " + &self.pieces[x][y].to_string();
            }
        }
        write!(f, "({})", str)
    }
}

impl Eq for Board<'_> {}

impl PartialEq for Board<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.pieces == other.pieces
    }
}

impl<'a> Board<'a> {
    pub fn from_colors(
        piece_set: &'a PieceSet,
        colors_up_north: [Option<[Color; 2]>; 9],
    ) -> Board<'a> {
        let mut pieces = [[Self::empty(); 3]; 3];
        for i in 0..9 {
            let row = i / 3;
            let col = i % 3;
            if let Some(colors) = colors_up_north[i] {
                let up = colors[0];
                let north = colors[1];
                let piece = piece_set.get_piece_index_from_colors(up, north);
                pieces[col][row] = piece.unwrap();
            }
        }
        Board { piece_set, pieces }
    }

    pub fn from_piece(piece_set: &'a PieceSet, piece_index: usize) -> Board<'a> {
        let mut pieces = [[Self::empty(); 3]; 3];
        for i in 0..9 {
            let row = i / 3;
            let col = i % 3;

            pieces[col][row] = piece_index;
        }
        pieces[1][1] = Self::empty();

        Board { piece_set, pieces }
    }

    pub fn piece(&self, x: usize, y: usize) -> usize {
        self.pieces[x][y]
    }

    pub fn coords_to_i8(coords: (usize, usize)) -> (i8, i8) {
        (coords.0 as i8, coords.1 as i8)
    }

    pub fn coords_to_usize(coords: (i8, i8)) -> (usize, usize) {
        (coords.0 as usize, coords.1 as usize)
    }

    fn children(&self) -> Vec<(Option<Board<'a>>, Direction)> {
        let empty_coords = self.empty_coords();
        Direction::posible_rotations()
            .iter()
            .map(|direction| {
                let coords = direction
                    .opposite()
                    .traslate(Self::coords_to_i8(empty_coords));
                let new_board = self.rotate(Self::coords_to_usize(coords), *direction);
                (new_board, *direction)
            })
            .collect()
    }

    pub fn children_filtered(&self) -> Vec<Board<'a>> {
        self.children_and_directions()
            .iter()
            .map(|pair| pair.0)
            .collect()
    }

    pub fn children_and_directions(&self) -> Vec<(Board<'a>, Direction)> {
        self.children()
            .iter()
            .filter(|c| c.0.is_some())
            .map(|c| (c.0.unwrap(), c.1))
            .collect()
    }

    pub fn ascii_art(&self) -> [[char; 11]; 11] {
        let mut b = [[' '; 11]; 11];

        for x in 0..3 {
            for y in 0..3 {
                let piece_index = self.pieces[x][y];
                if piece_index == Board::empty() {
                    continue;
                }

                let piece = self.piece_set.pieces[piece_index];

                let o = (1 + x * 3, 1 + y * 3);

                for direction in Direction::posible_rotations().iter() {
                    let p = direction.traslate((o.0 as i8, o.1 as i8));
                    let color = piece.color(*direction);

                    b[p.1 as usize + y][p.0 as usize + x] = color.letter();
                }

                let color = piece.color(Direction::Up);
                b[o.1 + y as usize][o.0 + x as usize] = color.letter();
            }
        }

        b
    }

    pub fn apply_moves_to_empty_position(&self, moves: &Vec<Direction>) -> Board<'a> {
        let mut ret = self.clone();
        moves
            .iter()
            .for_each(|d| ret = ret.move_empty_position(*d).unwrap());
        ret
    }

    pub fn infer_moves_to_empty_position(seq: Vec<Board<'a>>) -> Vec<Direction> {
        let mut ret: Vec<Direction> = Vec::new();
        for i in 0..seq.len() - 1 {
            let from = seq[i];
            let to = seq[i + 1];
            let candidates = from.children_and_directions();
            for c in candidates.iter() {
                if c.0 == to {
                    ret.push(c.1.opposite());
                }
            }
            assert_eq!(ret.len(), i + 1);
        }
        ret
    }

    pub fn ascii_art_string(&self) -> String {
        let ascii_art = self.ascii_art();
        let strings: Vec<String> = ascii_art.iter().map(|v| v.into_iter().collect()).collect();

        strings.join("\n")
    }

    pub fn from_one_piece(
        piece_set: &'a PieceSet,
        coords: (usize, usize),
        piece: usize,
    ) -> Board<'a> {
        let mut pieces = [[Self::empty(); 3]; 3];
        pieces[coords.0][coords.1] = piece;
        Board { piece_set, pieces }
    }

    pub fn from_initial(piece_set: &'a PieceSet, piece_index: usize) -> Board<'a> {
        let mut pieces = [[Self::empty(); 3]; 3];
        for x in 0..3 {
            for y in 0..3 {
                if x == 1 && y == 1 {
                    continue;
                }
                pieces[x][y] = piece_index;
            }
        }
        Board { piece_set, pieces }
    }

    pub fn empty() -> usize {
        99
    }

    pub fn is_empty(&self, coords: (usize, usize)) -> bool {
        self.pieces[coords.0][coords.1] == Self::empty()
    }

    pub fn empty_coords(&self) -> (usize, usize) {
        for x in 0..3 {
            for y in 0..3 {
                if self.is_empty((x, y)) {
                    return (x, y);
                }
            }
        }
        panic!("Sin vacio");
    }

    pub fn inside(&self, coords: (i8, i8)) -> bool {
        coords.0 >= 0
            && coords.0 < self.pieces.len() as i8
            && coords.1 >= 0
            && coords.1 <= self.pieces[0].len() as i8
    }

    pub fn move_empty_position(&self, d: Direction) -> Option<Board<'a>> {
        let empty = self.empty_coords();
        let o = d.opposite();
        let coords = Board::coords_to_usize(d.traslate(Board::coords_to_i8(empty)));
        self.rotate(coords, o)
    }

    pub fn rotate(&self, coords: (usize, usize), d: Direction) -> Option<Board<'a>> {
        //println!( "ROTATE: coords:{:?} d:{:?}", coords, d );
        //println!( "{}", self.ascii_art_string() );

        if coords.0 > 2 || coords.1 > 2 {
            return None;
        }

        if self.is_empty(coords) {
            return None;
        }

        let to = d.traslate((coords.0 as i8, coords.1 as i8));
        if !self.inside(to) {
            return None;
        }
        let to = (to.0 as usize, to.1 as usize);
        if !self.is_empty(to) {
            return None;
        }

        let old_piece = self.pieces[coords.0][coords.1];

        use Direction::*;

        let piece = match d {
            North | South => self.piece_set.rotate(old_piece, d),
            East | West => self.piece_set.rotate(old_piece, d.opposite()),
            _ => panic!("No se puede rotar así"),
        };

        let mut pieces = self.pieces.clone();
        pieces[coords.0][coords.1] = Self::empty();
        pieces[to.0][to.1] = piece;

        Some(Board {
            piece_set: self.piece_set,
            pieces,
        })
    }
}

#[cfg(test)]
mod tests {

    use crate::crossteaser::Board;
    use crate::crossteaser::Direction;
    use crate::crossteaser::Direction::*;
    use crate::crossteaser::Piece;
    use crate::crossteaser::PieceSet;

    #[test]
    fn rotate() {
        let p1 = Piece::seed();
        let p2 = p1.rotate(South);
        let p3 = p2.rotate(North);

        println!("Pieza original: {:?}", p1);
        println!("Pieza rotada: {:?}", p2);
        println!("Pieza rotada de vuelta: {:?}", p3);

        assert!(p1 == p3);

        assert!(!std::ptr::eq(&p2, &p3));
    }

    #[test]
    fn create_pieceset() {
        let p1 = Piece::seed();
        let piece_set = PieceSet::from_piece(&p1);
        println!("pieceSet:{:?}", piece_set);
        assert!(piece_set.pieces.len() == 6 * 4);
    }

    #[test]
    fn create_pieces() {
        let piece_set = PieceSet::from_piece(&Piece::seed());

        let i1 = 0;
        let i2 = piece_set.rotate(i1, East);
        let i3 = piece_set.rotate(i2, South);

        let p1 = piece_set.pieces[i1];
        let p2 = p1.rotate(East);
        let p3 = p2.rotate(South);

        assert!(piece_set.pieces[i1] == p1);
        assert!(piece_set.pieces[i2] == p2);
        assert!(piece_set.pieces[i3] == p3);
    }

    #[test]
    fn index_rotations_same_as_regular_pieces() {
        use rand::prelude::*;

        let piece_set = PieceSet::from_piece(&Piece::seed());

        let mut index = 0;
        let mut piece = piece_set.pieces[index];

        for _ in 0..100 {
            let d: usize = random::<usize>() % 4;
            let direction = Direction::from_index(d);
            index = piece_set.rotate(index, direction);
            piece = piece.rotate(direction);
        }

        assert!(piece == piece_set.pieces[index]);
    }

    #[cfg(not(debug_assertions))]
    #[test]
    fn index_faster_than_regular_pieces() {
        // ESTE TEST SOLO TIENE SENTIDO EN RELEASE
        fn measure_time<T>(msg: &str, function: &dyn Fn() -> T) -> (u128, T) {
            let now = std::time::Instant::now();
            let ret = function();
            let millis = now.elapsed().as_millis();
            println!("{}: {}", msg, millis);
            (millis, ret)
        };

        let piece_set = PieceSet::from_piece(&Piece::seed());
        let times: usize = 10000000;

        let (millis_regular, final_regular_piece) = measure_time("regular", &|| {
            let mut piece = piece_set.pieces[0];
            for i in 0..times {
                let direction = Direction::from_index(i % 4);
                piece = piece.rotate(direction);
            }
            piece
        });

        let (millis_index, final_index) = measure_time("index", &|| {
            let mut piece = 0;
            for i in 0..times {
                let direction = Direction::from_index(i % 4);
                piece = piece_set.rotate(piece, direction);
            }
            piece
        });

        assert!(final_regular_piece == piece_set.pieces[final_index]);
        assert!(millis_regular > millis_index);
    }

    #[test]
    fn one_piece_board_1() {
        let piece_set = PieceSet::from_piece(&Piece::seed());
        let piece_index = 0;

        let board1 = Board::from_one_piece(&piece_set, (0, 0), piece_index);
        assert!(board1.pieces[0][0] == piece_index);
        assert!(board1.pieces[0][1] == Board::empty());

        let board2 = board1.rotate((0, 0), South).unwrap();
        assert!(board2.pieces[0][0] == Board::empty());
        assert!(board2.pieces[0][1] == piece_set.rotate(piece_index, South));
    }

    #[test]
    fn one_piece_board_2() {
        let piece_set = PieceSet::from_piece(&Piece::seed());
        let piece_index = 0;

        let board1 = Board::from_one_piece(&piece_set, (0, 0), piece_index);
        let board2 = board1.rotate((0, 0), South).unwrap();

        let board3 =
            Board::from_one_piece(&piece_set, (0, 1), piece_set.rotate(piece_index, South));

        println!("BOARD1:\n{}", board1.ascii_art_string());
        println!("BOARD2:\n{}", board2.ascii_art_string());

        assert!(board2 == board3);
    }

    #[test]
    fn one_piece_board_3() {
        let piece_set = PieceSet::from_piece(&Piece::seed());
        let piece_index = 0;

        let mut board = Board::from_one_piece(&piece_set, (0, 0), piece_index);
        let mut coords: (i8, i8) = (0, 0);
        let directions = vec![South, South, East, East, North, North, West, West];

        println!("Original");
        println!("{}", board.ascii_art_string());

        directions.iter().for_each(|d| {
            board = board
                .rotate((coords.0 as usize, coords.1 as usize), *d)
                .unwrap();
            let new_coords = d.traslate(coords);
            coords = (new_coords.0 as i8, new_coords.1 as i8);
            println!("Tras {:?}", d);
            println!("{}", board.ascii_art_string());
        });

        assert!(board == Board::from_one_piece(&piece_set, (0, 0), piece_index));
    }

    #[test]
    fn initial_board() {
        let piece_set = PieceSet::from_piece(&Piece::seed());
        let board = Board::from_initial(&piece_set, 0);
        println!("{}", board.ascii_art_string());
        println!("");
        println!(
            "{}",
            board.rotate((1, 0), South).unwrap().ascii_art_string()
        );
        println!("");
        println!(
            "{}",
            board.rotate((1, 2), North).unwrap().ascii_art_string()
        );
        println!("");
        println!("{}", board.rotate((0, 1), East).unwrap().ascii_art_string());
        println!("");
        println!("{}", board.rotate((2, 1), West).unwrap().ascii_art_string());
        println!("");
    }

    #[test]
    fn initial_board_children() {
        let piece_set = PieceSet::from_piece(&Piece::seed());
        let board = Board::from_initial(&piece_set, 0);
        println!("{}", board.ascii_art_string());
        println!("");

        let children = board.children();
        println!("children:{}", children.len());
        assert!(children.len() == 4);

        let children = board.children_filtered();
        assert!(children.len() == 4);

        children.iter().for_each(|c| {
            println!("{}", c.ascii_art_string());
            println!("");
        });

        children.iter().for_each(|c| {
            assert!(c.children_filtered().len() == 3);
        });
    }

    #[test]
    fn moves_to_empty_position() {
        let piece_set = PieceSet::from_piece(&Piece::seed());
        let board = Board::from_initial(&piece_set, 0);

        let mut b1 = board.move_empty_position(Direction::South).unwrap();
        b1 = b1.move_empty_position(Direction::East).unwrap();
        println!("{}\n", b1.ascii_art_string());

        let b2 = board.apply_moves_to_empty_position(&vec![Direction::South, Direction::East]);
        println!("{}\n", b2.ascii_art_string());

        assert!(b1.pieces == b2.pieces)
    }
}
