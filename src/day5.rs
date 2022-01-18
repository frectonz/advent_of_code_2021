use std::cmp::{max, min};

struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn new(s: &str) -> Self {
        let (x, y) = s.split_once(",").unwrap();

        Self {
            x: x.parse::<usize>().unwrap(),
            y: y.parse::<usize>().unwrap(),
        }
    }
}

#[derive(Debug, PartialEq)]
struct Range {
    start: usize,
    end: usize,
}

#[derive(Debug)]
struct Line {
    x: Range,
    y: Range,
}

impl Line {
    fn new(data: &str) -> Line {
        let (start, end) = data
            .trim()
            .split_once("->")
            .map(|(start, end)| (start.trim(), end.trim()))
            .unwrap();

        let start = Point::new(start);
        let end = Point::new(end);

        let start_x = min(start.x, end.x);
        let end_x = max(start.x, end.x);

        let start_y = min(start.y, end.y);
        let end_y = max(start.y, end.y);

        Line {
            x: Range {
                start: start_x,
                end: (end_x + 1),
            },
            y: Range {
                start: start_y,
                end: (end_y + 1),
            },
        }
    }

    fn is_straight(&self) -> bool {
        self.x.start == self.x.end - 1 || self.y.start == self.y.end - 1
    }
}

pub fn get_day5_answer() {
    let lines = include_str!("../data/input_5.txt")
        .lines()
        .map(Line::new)
        .collect::<Vec<_>>();

    let (max_x, max_y) = lines.iter().fold((0, 0), |(mut x, mut y), line| {
        x = if x < line.x.end { line.x.end } else { x };
        y = if y < line.y.end { line.y.end } else { y };

        (x, y)
    });

    let mut grid = vec![vec![0; max_x]; max_y];

    let mut count = 0;

    lines
        .iter()
        .filter(|line| line.is_straight())
        .for_each(|line| {
            for x in line.x.start..line.x.end {
                for y in line.y.start..line.y.end {
                    grid[y][x] += 1;

                    if grid[y][x] == 2 {
                        count += 1;
                    }
                }
            }
        });

    println!("Day 5 Answer 1: {}", count);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_line() {
        let line = Line::new("1,1 -> 1,3");

        assert_eq!(line.x, Range { start: 1, end: 2 });
        assert_eq!(line.y, Range { start: 1, end: 3 });
    }
}
