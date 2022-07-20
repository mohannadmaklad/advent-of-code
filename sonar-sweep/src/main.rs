use std::fs;

fn main() {
    let input: Vec<u64> = fs::read_to_string("src/in.txt")
        .expect("Unable to read file")
        .trim()
        .split("\n")
        .map(|s| s.parse::<u64>().unwrap())
        .collect();

    println!("{}", part_one(&input));
    println!("{}", part_two(&input));
}

fn part_one(input: &Vec<u64>) -> u64 {
    input
        .iter()
        .zip(input.iter().skip(1))
        .filter(|(a, b)| b > a)
        .count() as u64
}

fn part_two(input: &Vec<u64>) -> u64 {
    let windows_sum = input
        .windows(3)
        .map(|window| window.iter().sum::<u64>())
        .collect::<Vec<u64>>();

    windows_sum
        .iter()
        .zip(windows_sum.iter().skip(1))
        .filter(|(a, b)| b > a)
        .count() as u64
}
