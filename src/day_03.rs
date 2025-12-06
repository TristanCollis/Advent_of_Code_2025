use common::*;

const EXAMPLE_PATH: &str = r"./day/03/example.txt";
const INPUT_PATH: &str = r"./day/03/input.txt";

pub fn part_1(use_example: bool) -> Result<()> {
    _part_1(format_input(parse_file(if use_example {
        &EXAMPLE_PATH
    } else {
        &INPUT_PATH
    })?))
}

pub fn part_2(use_example: bool) -> Result<()> {
    _part_2(format_input(parse_file(if use_example {
        &EXAMPLE_PATH
    } else {
        &INPUT_PATH
    })?))
}

type Input = Vec<Vec<u8>>;

fn _part_1(input: Input) -> Result<()> {
    let joltage: u64 = input
        .into_iter()
        .map(|bank| {
            let &tens_digit = bank[..bank.len() - 1].iter().max().unwrap();
            let left = (bank.iter().position(|&n| n == tens_digit).unwrap()) + 1;
            let &ones_digit = bank[left..].iter().max().unwrap();
            tens_digit as u64 * 10 + ones_digit as u64
        })
        .sum();

    dbg!(joltage);
    Ok(())
}

fn _part_2(input: Input) -> Result<()> {
    let joltages: Vec<u64> = input
        .iter()
        .map(|bank| joltage_per_bank(bank, 12))
        .collect();
    let joltage: u64 = joltages.iter().sum();
    dbg!(joltage);
    Ok(())
}

fn format_input(input: Vec<String>) -> Input {
    input
        .into_iter()
        .map(|line| {
            line.chars()
                .map(|character| character.to_string().parse::<u8>().unwrap())
                .collect()
        })
        .collect()
}

fn joltage_per_bank(bank: &Vec<u8>, digit_count: usize) -> u64 {
    let mut number = 0;
    let mut left = 0;
    let mut right = bank.len() - digit_count;

    for _ in 0..digit_count {
        let digit = bank[left..=right].iter().max().unwrap();

        left += bank[left..=right].iter().position(|elem| elem == digit).unwrap() + 1;
        right += 1;

        number = number * 10 + *digit as u64;
    }

    number
}
