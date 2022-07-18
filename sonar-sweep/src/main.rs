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
    let mut count: u64 = 0;
    for sample in input.iter().zip(input.iter().skip(1)) {
        if sample.1 > sample.0 {
            count += 1;
        }
    }
    count
}

fn part_two(input: &Vec<u64>) -> u64 {
    let mut count = 0;
    let windows_sum = input
        .windows(3)
        .map(|window| window.iter().sum::<u64>())
        .collect::<Vec<u64>>();

    for w in windows_sum.iter().zip(windows_sum.iter().skip(1)) {
        if w.1 > w.0 {
            count = count + 1;
        }
    }
    count
}
