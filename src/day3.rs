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
        let s: Vec<_> = s.split("").filter(|b| !b.is_empty()).collect();
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
    part1_answer();
    part2_answer();
}

fn part2_answer() {
    let oxygen_generator_rating = {
        let mut bits = read_to_string("data/input_3.txt")
            .unwrap()
            .lines()
            .map(|line| Bits::from_str(line).unwrap())
            .map(|bits| bits.0)
            .collect::<Vec<_>>();

        for (column_idx, _) in [0; COLUMN_SIZE].iter().enumerate().cycle() {
            if bits.len() == 1 {
                break;
            }

            let mut columns = Vec::with_capacity(COLUMN_SIZE);

            for bit in &bits {
                columns.push(bit[column_idx]);
            }

            let (most_common, _, equal) = count_booleans(&columns);

            let mut new_bits = vec![];

            if equal == true {
                for bit in bits {
                    if bit[column_idx] == true {
                        new_bits.push(bit);
                    }
                }
            } else if most_common == true {
                for bit in bits {
                    if bit[column_idx] == true {
                        new_bits.push(bit);
                    }
                }
            } else if most_common == false {
                for bit in bits {
                    if bit[column_idx] == false {
                        new_bits.push(bit);
                    }
                }
            }

            bits = new_bits;
        }

        bits
    };

    let co2_scrubber_rating = {
        let mut bits = read_to_string("data/input_3.txt")
            .unwrap()
            .lines()
            .map(|line| Bits::from_str(line).unwrap())
            .map(|bits| bits.0)
            .collect::<Vec<_>>();

        for (column_idx, _) in [0; COLUMN_SIZE].iter().enumerate().cycle() {
            if bits.len() == 1 {
                break;
            }

            let mut columns = Vec::with_capacity(COLUMN_SIZE);

            for bit in &bits {
                columns.push(bit[column_idx]);
            }

            let (_, least_common, equal) = count_booleans(&columns);

            let mut new_bits = vec![];

            if equal == true {
                for bit in bits {
                    if bit[column_idx] == false {
                        new_bits.push(bit);
                    }
                }
            } else if least_common == true {
                for bit in bits {
                    if bit[column_idx] == true {
                        new_bits.push(bit);
                    }
                }
            } else if least_common == false {
                for bit in bits {
                    if bit[column_idx] == false {
                        new_bits.push(bit);
                    }
                }
            }

            bits = new_bits;
        }

        bits
    };

    let oxygen_generator_rating = to_number(&oxygen_generator_rating[0]);
    let co2_scrubber_rating = to_number(&co2_scrubber_rating[0]);

    let ans = oxygen_generator_rating * co2_scrubber_rating;

    println!("Day 3 Answer 2: {}", ans);
}

fn part1_answer() {
    let bits = read_to_string("data/input_3.txt")
        .unwrap()
        .lines()
        .map(|line| Bits::from_str(line).unwrap())
        .map(|bits| bits.0)
        .collect::<Vec<_>>();

    let columns: Vec<Vec<bool>> = generate_columns(&bits);

    let (gamma_rate, epsilon_rate) = columns.iter().fold(
        (
            Vec::with_capacity(COLUMN_SIZE),
            Vec::with_capacity(COLUMN_SIZE),
        ),
        |(mut gamma_rate, mut epsilon_rate), column| {
            let (most_common, least_common, _) = count_booleans(column);

            gamma_rate.push(most_common);
            epsilon_rate.push(least_common);

            (gamma_rate, epsilon_rate)
        },
    );

    let gamma_rate = to_number(&gamma_rate);
    let epsilon_rate = to_number(&epsilon_rate);

    let ans = gamma_rate * epsilon_rate;

    println!("Day 3 Answer 1: {}", ans);
}

fn generate_columns(bits: &Vec<Vec<bool>>) -> Vec<Vec<bool>> {
    let mut columns: Vec<Vec<bool>> = Vec::with_capacity(COLUMN_SIZE);

    for (column_idx, _) in [0; COLUMN_SIZE].iter().enumerate() {
        let mut column: Vec<bool> = Vec::with_capacity(bits.len());

        for bit in bits {
            column.push(bit[column_idx]);
        }

        columns.push(column)
    }

    columns
}

fn to_number(booleans: &Vec<bool>) -> usize {
    let booleans = booleans
        .iter()
        .map(|b| match b {
            true => "1",
            false => "0",
        })
        .collect::<String>();

    usize::from_str_radix(booleans.as_str(), 2).unwrap()
}

fn count_booleans(readings: &Vec<bool>) -> (bool, bool, bool) {
    let (trues, falses) = readings.iter().fold((0, 0), |(mut trues, mut falses), r| {
        match r {
            true => trues += 1,
            false => falses += 1,
        };

        (trues, falses)
    });

    let most_common = trues > falses;
    let least_common = falses > trues;
    let equal = trues == falses;

    (most_common, least_common, equal)
}
