use std::{
    collections::HashSet,
    fmt::{Display, Formatter, Result as IoResult},
};

#[derive(Debug)]
struct RandomNumberGenerator {
    numbers: Vec<usize>,
    idx: usize,
}

impl RandomNumberGenerator {
    fn new(s: &str) -> Self {
        let numbers = s
            .split(",")
            .map(|str_num| str_num.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();

        Self { numbers, idx: 0 }
    }
}

impl Iterator for RandomNumberGenerator {
    type Item = usize;
    fn next(&mut self) -> Option<Self::Item> {
        let num = self.numbers.get(self.idx).and_then(|num| Some(*num));
        self.idx += 1;
        num
    }
}

#[derive(Debug, PartialEq)]
struct Cell {
    idx: usize,
    value: usize,
    status: CellStatus,
}

#[derive(Debug, PartialEq)]
enum CellStatus {
    MARKED,
    UNMARKED,
}

impl Cell {
    fn new((idx, value): (usize, usize)) -> Self {
        Self {
            idx,
            value,
            status: CellStatus::UNMARKED,
        }
    }

    fn is(&self, num: usize) -> bool {
        self.value == num
    }

    fn mark(&mut self) {
        self.status = CellStatus::MARKED;
    }

    fn is_marked(&self) -> bool {
        self.status == CellStatus::MARKED
    }

    fn is_unmarked(&self) -> bool {
        self.status == CellStatus::UNMARKED
    }
}

impl Display for Cell {
    fn fmt(&self, f: &mut Formatter<'_>) -> IoResult {
        match self.status {
            CellStatus::UNMARKED => write!(f, "{}", self.value),
            CellStatus::MARKED => write!(f, "*{}*", self.value),
        }
    }
}

const COLUMN_SIZE: usize = 5;

#[derive(Debug)]
struct Board {
    cells: Vec<Cell>,
}

impl Board {
    fn new(s: &str) -> Self {
        let cells = s
            .lines()
            .map(|line| line.split(" ").collect::<Vec<&str>>())
            .flatten()
            .filter(|s| s.len() > 0)
            .map(|num_str| num_str.parse::<usize>().unwrap())
            .enumerate()
            .map(Cell::new)
            .collect::<Vec<Cell>>();

        Self { cells }
    }

    fn mark(&mut self, num: usize) {
        if let Some(cell) = self.cells.iter_mut().find(|cell| cell.is(num)) {
            cell.mark();
        }
    }

    fn make_rows(&self) -> Vec<&[Cell]> {
        self.cells
            .split_inclusive(|cell| (cell.idx + 1) % 5 == 0)
            .collect::<Vec<&[Cell]>>()
    }

    fn make_columns(&self) -> Vec<Vec<&Cell>> {
        let mut columns = Vec::with_capacity(COLUMN_SIZE);

        let rows = self.make_rows();

        for (column_idx, _) in [0; COLUMN_SIZE].iter().enumerate() {
            let mut column = Vec::with_capacity(COLUMN_SIZE);

            for row in &rows {
                column.push(&row[column_idx]);
            }

            columns.push(column)
        }

        columns
    }

    fn is_completed(&self) -> bool {
        let at_least_one_row_completed = self
            .make_rows()
            .iter()
            .any(|row| row.iter().all(|cell| cell.is_marked()));

        let at_least_one_column_completed = self
            .make_columns()
            .iter()
            .any(|column| column.iter().all(|cell| cell.is_marked()));

        at_least_one_row_completed || at_least_one_column_completed
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut Formatter<'_>) -> IoResult {
        let rows = self.make_rows();
        let mut board = String::new();

        for row in rows {
            for cell in row {
                board.push_str(format!("{},", cell).as_str());
            }
            board.push_str("\n");
        }
        write!(f, "{}", board)
    }
}

pub fn get_day4_answer() {
    part1_answer();
    part2_answer()
}

fn part1_answer() {
    let input = include_str!("../data/input_4.txt")
        .split("\n\n")
        .collect::<Vec<&str>>();

    let random_numbers = RandomNumberGenerator::new(input[0]);

    let mut boards = input
        .iter()
        .skip(1)
        .map(|b_str| Board::new(b_str))
        .collect::<Vec<Board>>();

    for r in random_numbers {
        for b in &mut boards {
            b.mark(r);
            if b.is_completed() {
                let sum_of_unmarked_cells = b.cells.iter().fold(0, |mut acc, cell| {
                    if cell.is_unmarked() {
                        acc += cell.value;
                    }
                    acc
                });

                let ans = r * sum_of_unmarked_cells;

                println!("Day 4 Answer 1: {}", ans);
                return;
            }
        }
    }
}

fn part2_answer() {
    let input = include_str!("../data/input_4.txt")
        .split("\n\n")
        .collect::<Vec<&str>>();

    let random_numbers = RandomNumberGenerator::new(input[0]);

    let mut boards = input
        .iter()
        .skip(1)
        .map(|b_str| Board::new(b_str))
        .collect::<Vec<Board>>();

    let mut completed = HashSet::new();

    for r in random_numbers {
        for (idx, b) in &mut boards.iter_mut().enumerate() {
            b.mark(r);

            if b.is_completed() {
                completed.insert(idx);
            }

            if completed.len() == 100 {
                let sum_of_unmarked_cells = b.cells.iter().fold(0, |mut acc, cell| {
                    if cell.is_unmarked() {
                        acc += cell.value;
                    }
                    acc
                });

                let ans = r * sum_of_unmarked_cells;

                println!("Day 4 Answer 2: {}", ans);
                return;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Board, RandomNumberGenerator};

    #[test]
    fn test_random_number_generator() {
        let input = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1";
        let random_numbers = RandomNumberGenerator::new(input);
        let random_numbers = random_numbers
            .map(|num| num.to_string())
            .collect::<Vec<String>>()
            .join(",");

        assert_eq!(input, random_numbers);
    }

    #[test]
    fn test_parse_board() {
        let input = "22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19";
        let _board = Board::new(input);
        let _input = input.split("\n").collect::<Vec<&str>>().join("");
    }
}
