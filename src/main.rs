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

#[derive(Debug, Clone, PartialEq)]
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

    fn possible_moves(&self) -> Vec<Move> {
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

                if direction == Direction::Right && self.vertical_walls.get(current).is_empty() ||
                    direction == Direction::Left && self.vertical_walls.get(next).is_empty() ||
                    direction == Direction::Down && self.horizontal_walls.get(current).is_empty() ||
                    direction == Direction::Up && self.horizontal_walls.get(next).is_empty() {
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

                let current = Coordinate::new(x, y);
                for direction in vec![Direction::Up, Direction::Down, Direction::Left, Direction::Right] {
                    let next = current.move_to(direction);
                    if !next.inside(self.width, self.height) {
                        continue;
                    }

                    if direction == Direction::Right && self.vertical_walls.get(current).is_empty() ||
                        direction == Direction::Left && self.vertical_walls.get(next).is_empty() ||
                        direction == Direction::Down && self.horizontal_walls.get(current).is_empty() ||
                        direction == Direction::Up && self.horizontal_walls.get(next).is_empty() {
                        moves.push(Move {
                            destination: current,
                            place_wall: direction,
                        });
                    }
                }
            }
        }

        moves
    }

    fn game_over(&self) -> bool {
        //     the game is over when the green player can't reach the blue player
        let mut queue = Vec::new();
        let mut visited = vec![vec![false; self.width as usize]; self.height as usize];
        queue.push(self.blue_position);

        while !queue.is_empty() {
            let current = queue.remove(0);
            if visited[current.y as usize][current.x as usize] {
                continue;
            }
            if current == self.green_position {
                return false;
            }
            visited[current.y as usize][current.x as usize] = true;


            for direction in vec![Direction::Up, Direction::Down, Direction::Left, Direction::Right] {
                let next = current.move_to(direction);
                if !next.inside(self.width, self.height) {
                    continue;
                }

                if direction == Direction::Right && self.vertical_walls.get(current).is_empty() ||
                    direction == Direction::Left && self.vertical_walls.get(next).is_empty() ||
                    direction == Direction::Down && self.horizontal_walls.get(current).is_empty() ||
                    direction == Direction::Up && self.horizontal_walls.get(next).is_empty() {
                    queue.push(next);
                }
            }
        };

        true
    }

    fn make_move(&mut self, mv: Move) -> bool {
        // make the move
        // if mv is in possible_moves, then make the move and place

        // return true if the move is made, false otherwise

        if !self.possible_moves().contains(&mv) {
            return false;
        }

        if self.blue_turn {
            self.blue_position = mv.destination;
        } else {
            self.green_position = mv.destination;
        }

        match mv.place_wall {
            Direction::Up => {
                self.horizontal_walls.set(mv.destination.move_to(Direction::Up), if self.blue_turn { Cell::Blue } else { Cell::Green });
            }
            Direction::Down => {
                self.horizontal_walls.set(mv.destination, if self.blue_turn { Cell::Blue } else { Cell::Green });
            }
            Direction::Left => {
                self.vertical_walls.set(mv.destination.move_to(Direction::Left), if self.blue_turn { Cell::Blue } else { Cell::Green });
            }
            Direction::Right => {
                self.vertical_walls.set(mv.destination, if self.blue_turn { Cell::Blue } else { Cell::Green });
            }
        }

        self.blue_turn = !self.blue_turn;
        true
    }
}

fn main() {
    let mut game = Game::new(7, 7);

    game.print();

    game.make_move(Move {
        destination: Coordinate::new(0, 1),
        place_wall: Direction::Down,
    });

    game.print();

    game.make_move(Move {
        destination: Coordinate::new(5, 5),
        place_wall: Direction::Right,
    });

    game.print();

    // println!("Game over: {}", game.game_over());

    // game.horizontal_walls.set(Coordinate::new(0, 0), Cell::Blue);
    // game.print();
    //
    // println!("Game over: {}", game.game_over());
}