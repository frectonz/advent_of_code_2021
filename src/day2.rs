use std::{fs::read_to_string, num::ParseIntError, str::FromStr};

#[derive(Debug)]
enum Command {
    Forward(usize),
    Down(usize),
    Up(usize),
}

#[derive(Debug)]
enum CommandParseError {
    IncorrectFormat,
    ParseIntError,
    UnknownCommand,
}

impl From<ParseIntError> for CommandParseError {
    fn from(_: ParseIntError) -> Self {
        Self::ParseIntError
    }
}

impl FromStr for Command {
    type Err = CommandParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s: Vec<&str> = s.split(" ").collect();

        if s.len() != 2 {
            return Err(Self::Err::IncorrectFormat);
        }

        let cmd = s[0];
        let num = s[1].parse::<usize>()?;

        match cmd {
            "forward" => Ok(Self::Forward(num)),
            "down" => Ok(Self::Down(num)),
            "up" => Ok(Self::Up(num)),
            _ => Err(Self::Err::UnknownCommand),
        }
    }
}

pub fn get_day2_answer() {
    let mut h: usize = 0;
    let mut d: usize = 0;
    let mut aim: usize = 0;

    read_to_string("data/input_2.txt")
        .unwrap()
        .lines()
        .map(|line| Command::from_str(line).unwrap())
        .for_each(|cmd| {
            use Command::*;
            match cmd {
                Forward(num) => {
                    h += num;
                    d += aim * num;
                }
                Down(num) => aim += num,
                Up(num) => aim -= num,
            };
        });

    println!("Day 2 Answer: {}", h * d);
}
