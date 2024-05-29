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

    fn values() -> Vec<Direction> {
        vec![Direction::Up, Direction::Down, Direction::Left, Direction::Right]
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

impl Move {
    fn new(destination: Coordinate, place_wall: Direction) -> Move {
        Move { destination, place_wall }
    }


    fn from_notation(notation: &str) -> Move {
        let destination = Coordinate::new(
            notation.chars().nth(0).unwrap() as i32 - 'A' as i32,
            7 - notation.chars().nth(1).unwrap() as i32 + '0' as i32,
        );
        let place_wall = match notation.chars().nth(2).unwrap() {
            'U' => Direction::Up,
            'D' => Direction::Down,
            'L' => Direction::Left,
            'R' => Direction::Right,
            _ => panic!("Invalid move notation"),
        };

        Move::new(destination, place_wall)
    }
}

#[derive(Debug)]
struct Score {
    blue: i32,
    green: i32,
}

#[derive(Debug)]
enum Winner {
    Blue,
    Green,
    Draw,
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

        println!("  ┌───┬───┬───┬───┬───┬───┬───┐");
        for y in 0..self.width {
            print!("{} │ ", 7 - y);

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
                print!("  ├");
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
        println!("  └───┴───┴───┴───┴───┴───┴───┘");
        println!("    A   B   C   D   E   F   G");
    }

    fn reachable_positions(&self, start: Coordinate, step: i32) -> Vec<Vec<bool>> {
        let mut reachable = vec![vec![false; self.width as usize]; self.height as usize];
        let mut queue = Vec::new();
        queue.push((start, 0));

        while !queue.is_empty() {
            let (current, current_step) = queue.remove(0);
            if current_step == step {
                continue;
            }

            for direction in Direction::values() {
                let next = current.move_to(direction);
                if !next.inside(self.width, self.height) {
                    continue;
                }

                if reachable[next.y as usize][next.x as usize] {
                    continue;
                }

                if direction == Direction::Right && self.vertical_walls.get(current).is_empty() ||
                    direction == Direction::Left && self.vertical_walls.get(next).is_empty() ||
                    direction == Direction::Down && self.horizontal_walls.get(current).is_empty() ||
                    direction == Direction::Up && self.horizontal_walls.get(next).is_empty() {
                    queue.push((next, current_step + 1));
                    reachable[next.y as usize][next.x as usize] = true;
                }
            }
        }

        reachable
    }

    fn possible_moves(&self) -> Vec<Move> {
        let mut moves = Vec::new();
        let start = if self.blue_turn {
            self.blue_position
        } else {
            self.green_position
        };

        let reachable = self.reachable_positions(start, 3);

        // get all possible moves
        // the wall placement is possible if the wall is not already placed and the edge is not on the border

        for y in 0..self.height {
            for x in 0..self.width {
                if !reachable[y as usize][x as usize] {
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

        let cell = if self.blue_turn { Cell::Blue } else { Cell::Green };

        match mv.place_wall {
            Direction::Up => self.horizontal_walls.set(mv.destination.move_to(Direction::Up), cell),
            Direction::Down => self.horizontal_walls.set(mv.destination, cell),
            Direction::Left => self.vertical_walls.set(mv.destination.move_to(Direction::Left), cell),
            Direction::Right => self.vertical_walls.set(mv.destination, cell),
        }

        self.blue_turn = !self.blue_turn;
        true
    }

    fn game_over(&self) -> bool {
        //     the game is over when the green player can't reach the blue player

        let blue_reachable = self.reachable_positions(self.blue_position, self.height * self.height);

        if !blue_reachable[self.green_position.y as usize][self.green_position.x as usize] {
            return true;
        }
        false
    }

    fn game_result(&self) -> (Winner, Score) {
        // the score is the area of the player can reach

        let blue_reachable = self.reachable_positions(self.blue_position, self.height * self.height);
        let green_reachable = self.reachable_positions(self.green_position, self.height * self.height);

        let mut blue_score = 0;
        let mut green_score = 0;

        for y in 0..self.height {
            for x in 0..self.width {
                if blue_reachable[y as usize][x as usize] {
                    blue_score += 1;
                }
                if green_reachable[y as usize][x as usize] {
                    green_score += 1;
                }
            }
        };

        let score = Score { blue: blue_score, green: green_score };

        match blue_score.cmp(&green_score) {
            std::cmp::Ordering::Greater => (Winner::Blue, score),
            std::cmp::Ordering::Less => (Winner::Green, score),
            std::cmp::Ordering::Equal => (Winner::Draw, score),
        }
    }
}

fn main() {
    let mut game = Game::new(7, 7);

    loop {
        game.print();
        println!("Now it's {}'s turn", if game.blue_turn { "Blue" } else { "Green" });

        loop {
            println!("Enter your move: ");
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();

            let mv = Move::from_notation(input.trim());
            let res = game.make_move(mv);
            if res {
                break;
            }
            println!("Invalid move");
        }

        if game.game_over() {
            game.print();
            let (winner, score) = game.game_result();
            println!("Game over, {:?} wins!", winner);
            println!("{0} - {1}", score.blue, score.green);

            break;
        }
    }
}