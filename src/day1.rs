use std::fs::read_to_string;

pub fn get_readings() -> Vec<usize> {
    read_to_string("data/input_1.txt")
        .unwrap()
        .lines()
        .map(|line| line.parse::<usize>().unwrap())
        .collect::<Vec<usize>>()
}

pub fn count_increases(numbers: &Vec<usize>) -> usize {
    use std::cmp::Ordering;

    numbers
        .iter()
        .enumerate()
        .skip(1)
        .map(|(idx, current)| current.cmp(&numbers[idx - 1]))
        .filter(|o| *o == Ordering::Greater)
        .count()
}

pub fn get_day1_answer() {
    let readings = get_readings();

    println!("Day 1 Answer 1: {} increased", count_increases(&readings));

    let measurements = readings
        .iter()
        .enumerate()
        .map(|(idx, num)| {
            *num + *readings.get(idx + 1).unwrap_or(&0) + *readings.get(idx + 2).unwrap_or(&0)
        })
        .collect::<Vec<usize>>();

    println!(
        "Day 1 Answer 2: {} increased",
        count_increases(&measurements)
    );
}
