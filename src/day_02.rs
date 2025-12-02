use std::ops::RangeInclusive;

use common::*;

const EXAMPLE_PATH: &str =
    r"C:/Users/trist/Documents/Github/AdventOfCode/aoc_2025\./day/02\./example.txt";
const INPUT_PATH: &str =
    r"C:/Users/trist/Documents/Github/AdventOfCode/aoc_2025\./day/02\./input.txt";

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

type Input = Vec<RangeInclusive<u64>>;

fn _part_1(input: Input) -> Result<()> {
    let id_sum = input
        .into_iter()
        .map(|range| {
            range
                .filter_map(|id| {
                    let id_str = id.to_string();

                    if id_str.len() % 2 == 1 {
                        return None;
                    }

                    if id_str[..(id_str.len() / 2)] != id_str[(id_str.len() / 2)..] {
                        return None;
                    }

                    Some(id)
                })
                .sum::<u64>()
        })
        .sum::<u64>();

    dbg!(id_sum);

    Ok(())
}


fn _part_2(input: Input) -> Result<()> {
    let id_sum = input
        .into_iter()
        .map(|range| {
            range
                .filter_map(|id| {
                    let id_str = id.to_string();

                    for slice_length in 1..=(id_str.len() / 2) {
                        if id_str.len() % slice_length != 0 {
                            continue;
                        }

                        if id_str.replace(&id_str[..slice_length], "") == "" {
                            return Some(id);
                        }
                    }

                    None
                })
                .sum::<u64>()
        })
        .sum::<u64>();

    dbg!(id_sum);
    Ok(())
}

fn format_input(input: Vec<String>) -> Input {
    input
        .first()
        .unwrap()
        .split(",")
        .map(
            |range_bounds| match range_bounds.split("-").into_iter().collect::<Vec<_>>()[..] {
                [start, end] => start.parse().unwrap()..=end.parse().unwrap(),
                _ => panic!(),
            },
        )
        .collect()
}
