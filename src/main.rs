use std::cmp::PartialEq;
use std::ops::Add;
use std::fmt;

#[derive(Clone)]
enum Cell {
    Empty,
    Blue,
    Green,
}

impl Cell {
    fn is_empty(&self) -> bool {
        match self {
            Cell::Empty => true,
            _ => false,
        }
    }
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Cell::Empty => write!(f, "_"),
            Cell::Blue => write!(f, "B"),
            Cell::Green => write!(f, "G"),
        }
    }
}

impl fmt::Debug for Cell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Cell::Empty => write!(f, " "),
            Cell::Blue | Cell::Green => write!(f, "■"),
        }
    }
}


struct Board {
    board_matrix: Vec<Vec<Cell>>,
}

impl Board {
    fn new(width: i32, height: i32) -> Board {
        Board {
            board_matrix: vec![vec![Cell::Empty; width as usize]; height as usize],
        }
    }

    fn get(&self, coordinate: Coordinate) -> &Cell {
        &self.board_matrix[coordinate.y as usize][coordinate.x as usize]
    }

    fn set(&mut self, coordinate: Coordinate, cell: Cell) {
        self.board_matrix[coordinate.y as usize][coordinate.x as usize] = cell;
    }

    fn print(&self) {
        for row in &self.board_matrix {
            for cell in row {
                print!("{:?} ", cell);
            }
            println!();
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn relative_position(self) -> Coordinate {
        match self {
            Direction::Up => Coordinate::new(0, -1),
            Direction::Down => Coordinate::new(0, 1),
            Direction::Left => Coordinate::new(-1, 0),
            Direction::Right => Coordinate::new(1, 0),
        }
    }

    fn horizontal(self) -> bool {
        match self {
            Direction::Left | Direction::Right => true,
            Direction::Up | Direction::Down => false,
        }
    }
}


impl PartialEq for Direction {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Direction::Up, Direction::Up) => true,
            (Direction::Down, Direction::Down) => true,
            (Direction::Left, Direction::Left) => true,
            (Direction::Right, Direction::Right) => true,
            _ => false,
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Coordinate {
    x: i32,
    y: i32,
}

impl Coordinate {
    fn new(x: i32, y: i32) -> Coordinate {
        Coordinate { x, y }
    }

    fn inside(self, width: i32, height: i32) -> bool {
        self.x >= 0 && self.x < width && self.y >= 0 && self.y < height
    }

    fn move_to(self, direction: Direction) -> Coordinate {
        self + direction.relative_position()
    }

    fn distance(self, other: Coordinate) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

impl PartialEq for Coordinate {
    fn eq(&self, other: &Coordinate) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Add<Coordinate> for Coordinate {
    type Output = Coordinate;

    fn add(self, other: Coordinate) -> Coordinate {
        Coordinate::new(self.x + other.x, self.y + other.y)
    }
}

#[derive(Debug)]
struct Move {
    destination: Coordinate,
    place_wall: Direction,
}


struct Game {
    width: i32,
    height: i32,

    blue_position: Coordinate,
    green_position: Coordinate,

    horizontal_walls: Board, // 0 for no wall, 1 for blue wall, 2 for green wall
    vertical_walls: Board,

    move_flag: bool, // true for move, false for place wall
    blue_turn: bool, // true for blue, false for green
}


impl Game {
    fn new(width: i32, height: i32) -> Game {
        Game {
            width,
            height,
            blue_position: Coordinate::new(0, 0), // the top-left corner
            green_position: Coordinate::new(width - 1, height - 1), // bottom-right corner
            horizontal_walls: Board::new(width, height - 1),
            vertical_walls: Board::new(width - 1, height),
            move_flag: true,
            blue_turn: true,
        }
    }

    fn print(&self) {
        // print the cells and walls
        // use table characters

        println!("┌───┬───┬───┬───┬───┬───┬───┐");
        for y in 0..self.width {
            print!("│ ");

            for x in 0..self.height {
                let cell_coordinate = Coordinate::new(x, y);

                if self.blue_position == cell_coordinate {
                    print!("B");
                } else if self.green_position == cell_coordinate {
                    print!("G");
                } else {
                    print!(" ");
                }

                if x < self.height - 1 {
                    if self.vertical_walls.get(cell_coordinate).is_empty() {
                        print!("   ");
                    } else {
                        print!(" ┃ ");
                    }
                }
            }

            print!(" │");
            println!();

            if y < self.width - 1 {
                print!("├");
                for x in 0..self.height {
                    let cell_coordinate = Coordinate::new(x, y);

                    if x != 0 { print!("┼"); }
                    if self.horizontal_walls.get(cell_coordinate).is_empty() == true {
                        print!("   ");
                    } else {
                        print!("━━━");
                    }
                }
                print!("┤");

                println!();
            }
        }
        println!("└───┴───┴───┴───┴───┴───┴───┘");
    }

    fn get_possible_moves(&self) -> Vec<Move> {
        let mut moves = Vec::new();
        let start = if self.blue_turn {
            self.blue_position
        } else {
            self.green_position
        };

        // bfs to find all possible moves(not include placing wall) in 3 steps
        let mut queue = Vec::new();
        let mut visited = vec![vec![false; self.width as usize]; self.height as usize];
        queue.push((start, 0));

        while !queue.is_empty() {
            let (current, step) = queue.remove(0);
            if visited[current.y as usize][current.x as usize] {
                continue;
            }
            visited[current.y as usize][current.x as usize] = true;

            if step == 3 {
                continue;
            }

            for direction in vec![Direction::Up, Direction::Down, Direction::Left, Direction::Right] {
                let next = current.move_to(direction);
                if !next.inside(self.width, self.height) {
                    continue;
                }

                if self.blue_turn && next == self.green_position ||
                    !self.blue_turn && next == self.blue_position {
                    continue;
                }

                if self.vertical_walls.get(current).is_empty() && direction == Direction::Right ||
                    self.vertical_walls.get(next).is_empty() && direction == Direction::Left ||
                    self.horizontal_walls.get(current).is_empty() && direction == Direction::Down ||
                    self.horizontal_walls.get(next).is_empty() && direction == Direction::Up {
                    queue.push((next, step + 1));
                }
            }
        };


        // get all possible moves
        // the wall placement is possible if the wall is not already placed and the edge is not on the border

        for y in 0..self.height {
            for x in 0..self.width {
                if visited[y as usize][x as usize] == false {
                    continue;
                }

                let coordinate = Coordinate::new(x, y);
                for direction in vec![Direction::Up, Direction::Down, Direction::Left, Direction::Right] {
                    let next = coordinate.move_to(direction);
                    if !next.inside(self.width, self.height) {
                        continue;
                    }

                    if self.vertical_walls.get(coordinate).is_empty() && direction == Direction::Right ||
                        self.vertical_walls.get(next).is_empty() && direction == Direction::Left ||
                        self.horizontal_walls.get(coordinate).is_empty() && direction == Direction::Down ||
                        self.horizontal_walls.get(next).is_empty() && direction == Direction::Up {
                        moves.push(Move {
                            destination: coordinate,
                            place_wall: direction,
                        });
                    }
                }
            }
        }

        moves
    }
}

fn main() {
    let mut game = Game::new(7, 7);

    // game.horizontal_walls.set(0, 0, Cell::Blue);
    game.horizontal_walls.set(Coordinate::new(0, 1), Cell::Green);
    game.vertical_walls.set(Coordinate::new(0, 0), Cell::Blue);
    // game.vertical_walls.set(Coordinate::new(0, 1), Cell::Blue);

    game.green_position = Coordinate::new(0, 1);
    game.print();

    println!("{:?}", game.get_possible_moves());
}