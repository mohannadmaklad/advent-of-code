use std::fs;

fn main() {
    part_one();
}

fn part_one() {
    let input: Vec<Vec<u64>> = fs::read_to_string("src//in.txt")
        .expect("Could not read file")
        .split('\n')
        .map(|x| {
            x.chars()
                .map(|c| c.to_string().parse::<u64>().unwrap())
                .collect()
        })
        .collect();

    let mut gamma_bits = input[0].clone();

    // Count the 1s for each column
    input.iter().skip(1).for_each(|row| {
        row.iter().enumerate().for_each(|(index, bit)| {
            gamma_bits[index] += bit;
        });
    });

    // Maximum number of bits
    let count = input.len() as u64;
    let gamma: String = gamma_bits
        .iter()
        .map(|bit| (bit / (count / 2)).to_string())
        .collect::<String>();
    let gamma_decimal = u64::from_str_radix(&gamma, 2).unwrap();

    let epsilon_bits = gamma
        .chars()
        .map(|bit| if bit == '1' { '0' } else { '1' })
        .collect::<String>();
    let epsilon_decimal = u64::from_str_radix(&epsilon_bits, 2).unwrap();

    println!("{:?}", gamma_decimal);
    println!("{:?}", epsilon_decimal);
    println!("Power Consumption: {}", gamma_decimal * epsilon_decimal);
}
