use common::*;

const EXAMPLE_PATH: &str =
    r"C:/Users/trist/Documents/Github/AdventOfCode/aoc_2025\./day/01\./example.txt";
const INPUT_PATH: &str =
    r"C:/Users/trist/Documents/Github/AdventOfCode/aoc_2025\./day/01\./input.txt";

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

type Input = Vec<i16>;

fn _part_1(input: Input) -> Result<()> {
    let mut readout = 50;
    let password = input
        .into_iter()
        .filter_map(|turn| {
            readout = (readout + turn) % 100;
            if readout == 0 {
                Some(())
            } else {
                None
            }
        })
        .count();

    dbg!(password);
    Ok(())
}

fn _part_2(input: Input) -> Result<()> {
    let mut readout: i32 = 50;
    let mut count: u32 = 0;

    for turn in input {
        let (range, inc) = {
            if turn > 0 {
                (0..turn, 1)
            } else {
                (turn..0, -1)
            }
        };

        for _ in range {
            readout = (readout + inc).rem_euclid(100);
            if readout == 0 {
                count += 1;
            }
        }
    }

    dbg!(count);

    Ok(())
}

fn format_input(input: Vec<String>) -> Input {
    input
        .iter()
        .map(|line| {
            line[1..].parse::<i16>().unwrap() * {
                if line.starts_with("L") {
                    -1
                } else {
                    1
                }
            }
        })
        .collect::<Vec<_>>()
}
