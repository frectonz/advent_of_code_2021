use std::{fs::read_to_string, ops::Index};

#[derive(PartialEq, Debug)]
enum Changes {
    Increased,
    Decreased,
}

pub fn get_answer() {
    let data = read_to_string("data/day1.txt")
        .unwrap()
        .lines()
        .map(|line| line.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();

    let mut last = data.index(1);
    let mut changes: Vec<Changes> = Vec::with_capacity(data.len());

    for current in data.iter().skip(1) {
        match current {
            current if current > last => {
                last = current;
                changes.push(Changes::Increased);
            }
            current if current < last => {
                last = current;
                changes.push(Changes::Decreased);
            }
            _ => continue,
        }
    }

    let increases = changes
        .iter()
        .filter(|c| **c == Changes::Increased)
        .collect::<Vec<&Changes>>()
        .len();

    println!("Day 1 Answer: {} increased", increases);
}
