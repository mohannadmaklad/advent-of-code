use std::fs;

fn main() {
    let _position = (0, 0, 0);
    let input: Vec<Vec<String>> = fs::read_to_string("in.txt")
        .expect("Unable to read file")
        .trim()
        .split("\n")
        .map(|s| s.split(" ").map(|s| s.to_string()).collect::<Vec<String>>())
        .collect();

    println!("{:?}", part_one(&input));
    println!("{:?}", part_two(&input));
}

fn part_one(input: &Vec<Vec<String>>) -> i64 {
    let pos = input
        .iter()
        .fold((0, 0), |position, command| match command[0].as_str() {
            "forward" => (position.0 + command[1].parse::<i64>().unwrap(), position.1),

            "up" => (position.0, position.1 - command[1].parse::<i64>().unwrap()),

            "down" => (position.0, position.1 + command[1].parse::<i64>().unwrap()),

            _ => position,
        });

    pos.0 * pos.1
}

fn part_two(input: &Vec<Vec<String>>) -> i64 {
    let pos = input
        .iter()
        .fold((0, 0, 0), |position, command| match command[0].as_str() {
            "forward" => (
                position.0 + command[1].parse::<i64>().unwrap(),
                position.1 + (position.2 * command[1].parse::<i64>().unwrap()),
                position.2,
            ),

            "up" => (
                position.0,
                position.1,
                position.2 - command[1].parse::<i64>().unwrap(),
            ),

            "down" => (
                position.0,
                position.1,
                position.2 + command[1].parse::<i64>().unwrap(),
            ),

            _ => position,
        });

    pos.0 * pos.1
}
