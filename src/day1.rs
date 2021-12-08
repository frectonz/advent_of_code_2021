use crate::utils::count_increases;
use crate::utils::get_readings;

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
