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
