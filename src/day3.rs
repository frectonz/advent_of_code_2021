use std::{fs::read_to_string, str::FromStr, usize};

#[derive(Debug)]
struct Bits(Vec<bool>);

#[derive(Debug)]
enum BitsParseError {
    UnknownBit,
}

impl FromStr for Bits {
    type Err = BitsParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s: Vec<&str> = s.split("").filter(|b| !b.is_empty()).collect();
        let mut bits: Vec<bool> = Vec::with_capacity(s.len());

        for bit in s {
            match bit {
                "0" => bits.push(false),
                "1" => bits.push(true),
                _ => return Err(Self::Err::UnknownBit),
            };
        }

        Ok(Self(bits))
    }
}

const COLUMN_SIZE: usize = 12;

pub fn get_day3_answer() {
    let bits = read_to_string("data/input_3.txt")
        .unwrap()
        .lines()
        .map(|line| Bits::from_str(line).unwrap())
        .map(|bits| bits.0)
        .collect::<Vec<_>>();

    let mut columns: Vec<Vec<bool>> = Vec::with_capacity(COLUMN_SIZE);

    for (column_idx, _) in [0; COLUMN_SIZE].iter().enumerate() {
        let mut column: Vec<bool> = Vec::with_capacity(bits.len());

        for bit in &bits {
            column.push(bit[column_idx]);
        }

        columns.push(column)
    }

    let mut gamma_rate = Vec::with_capacity(COLUMN_SIZE);
    let mut epsilon_rate = Vec::with_capacity(COLUMN_SIZE);
    for column in columns {
        let BooleanResult {
            most_common,
            least_common,
        } = count_booleans(column);

        gamma_rate.push(most_common);
        epsilon_rate.push(least_common);
    }

    let gamma_rate = to_number(gamma_rate);
    let epsilon_rate = to_number(epsilon_rate);

    let ans = gamma_rate * epsilon_rate;

    println!("Day 3 Answer 1: {}", ans);
}

fn to_number(booleans: Vec<bool>) -> usize {
    let booleans = booleans
        .iter()
        .map(|b| match b {
            true => "1",
            false => "0",
        })
        .collect::<String>();

    usize::from_str_radix(booleans.as_str(), 2).unwrap()
}

struct BooleanResult {
    most_common: bool,
    least_common: bool,
}

fn count_booleans(readings: Vec<bool>) -> BooleanResult {
    let mut trues = 0;
    let mut falses = 0;

    for b in readings {
        match b {
            true => trues += 1,
            false => falses += 1,
        };
    }

    let most_common = trues > falses;
    let least_common = !most_common;

    BooleanResult {
        most_common,
        least_common,
    }
}
