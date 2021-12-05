use std::fs;

pub trait AbstractPuzzle {
    fn get_day(&self) -> u8;
    fn solve_part_1(&self) -> String;
    fn solve_part_2(&self) -> String;
}

pub struct Puzzle04 {
    input: String,
}

impl AbstractPuzzle for Puzzle04 {
    fn get_day(&self) -> u8 {
        4
    }

    fn solve_part_1(&self) -> String {
        let parsed_input = ParsedInput::parse(self.input.as_str());
        let numbers = parsed_input.numbers;
        let mut boards = parsed_input.boards;
        for number in numbers {
            for board in boards.iter_mut() {
                board.add_number(number);
                if board.won {
                    return board.score().to_string();
                }
            }
        }
        unreachable!()
    }

    fn solve_part_2(&self) -> String {
        let parsed_input = ParsedInput::parse(self.input.as_str());
        let numbers = parsed_input.numbers;
        let mut boards = parsed_input.boards;
        let length = boards.len();
        for number in numbers {
            for i in 0..length {
                boards[i].add_number(number);
                let mut all_won = true;
                for board in boards.iter() {
                    if !board.won {
                        all_won = false;
                        break;
                    }
                }
                if all_won {
                    return boards[i].score().to_string();
                }
            }
        }
        unreachable!();
    }
}

impl Puzzle04 {
    pub fn create(input: &str) -> Box<dyn AbstractPuzzle> {
        Box::new(Puzzle04 {
            input: input.to_string(),
        })
    }
}

struct ParsedInput {
    numbers: Vec<u32>,
    boards: Vec<BingoBoard>,
}

impl ParsedInput {
    fn parse(input: &str) -> ParsedInput {
        let mut lines = input.lines();
        let numbers = lines
            .next()
            .unwrap()
            .split(',')
            .map(|s| s.parse::<u32>().unwrap())
            .collect::<Vec<u32>>();

        let boards = lines
            .map(|line| {
                line.split_whitespace()
                    .map(|token| token.parse::<u32>().unwrap())
                    .collect::<Vec<u32>>()
            })
            .flatten()
            .collect::<Vec<u32>>()
            .chunks(25)
            .into_iter()
            .map(BingoBoard::new)
            .collect::<Vec<BingoBoard>>();

        ParsedInput { numbers, boards }
    }
}

struct BingoBoard {
    won: bool,
    last_number: u32,
    board: [[u32; 5]; 5],
    marks: [[bool; 5]; 5],
}

impl BingoBoard {
    fn new(numbers: &[u32]) -> BingoBoard {
        let mut board: [[u32; 5]; 5] = [[0; 5]; 5];
        let mut row = 0;
        let mut col = 0;
        for number in numbers {
            board[row][col] = *number;
            col += 1;
            if col == 5 {
                row += 1;
                col = 0;
            }
        }
        BingoBoard {
            won: false,
            last_number: 0,
            board,
            marks: [[false; 5]; 5],
        }
    }

    fn add_number(&mut self, number: u32) {
        if self.won {
            return;
        }
        self.last_number = number;
        for row in 0..5 {
            for col in 0..5 {
                if self.board[row][col] == number {
                    self.marks[row][col] = true;
                }
            }
        }
        for i in 0..5 {
            let mut row = true;
            let mut col = true;
            for j in 0..5 {
                if !self.marks[i][j] {
                    row = false;
                }
                if !self.marks[j][i] {
                    col = false;
                }
            }
            if row || col {
                self.won = true;
                return;
            }
        }
    }

    fn score(&self) -> u32 {
        let mut sum = 0;
        for row in 0..5 {
            for col in 0..5 {
                if !self.marks[row][col] {
                    sum += self.board[row][col];
                }
            }
        }
        sum * self.last_number
    }
}

fn main() {
    let content = fs::read_to_string("./day04a/input.txt").unwrap();
    let puzzle = Puzzle04::create(&content);
    let day = format!("{:02}", puzzle.get_day());
    println!("Day {} Part 2: {}", day, puzzle.solve_part_2());
}
