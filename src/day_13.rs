use common::*;

const EXAMPLE_PATH: &str =
    r"C:/Users/trist/Documents/Github/AdventOfCode/aoc_2025\./day/13\./example.txt";
const INPUT_PATH: &str =
    r"C:/Users/trist/Documents/Github/AdventOfCode/aoc_2025\./day/13\./input.txt";

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

type Input = ();

fn _part_1(_input: Input) -> Result<()> {
    todo!();
}

fn _part_2(_input: Input) -> Result<()> {
    todo!()
}

fn format_input(_input: Vec<String>) -> Input {
    todo!()
}
