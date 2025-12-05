use std::ops::RangeInclusive;

use common::*;

const EXAMPLE_PATH: &str =
    r"./day/05/example.txt";
const INPUT_PATH: &str =
    r"./day/05/input.txt";

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

type Input = (Vec<RangeInclusive<u64>>, Vec<u64>);

fn _part_1((ranges, ids): Input) -> Result<()> {
    let fresh_ids = ids
        .iter()
        .filter(|&id| ranges.iter().any(|range| range.contains(id)))
        .count();

    dbg!(fresh_ids);

    Ok(())
}

fn _part_2((ranges, _): Input) -> Result<()> {
    let mut range_bounds: Vec<(u64, u64)> = vec![];

    for mut range in ranges.into_iter().map(|range| range.into_inner()) {
        let mut removal_indices: Vec<usize> = vec![];

        for (i, existing_range) in range_bounds.iter().enumerate() {
            if !(range.0 <= existing_range.1 && existing_range.0 <= range.1) {
                continue;
            }

            range = (range.0.min(existing_range.0), range.1.max(existing_range.1));
            removal_indices.push(i);
        }

        for i in removal_indices.iter().rev() {
            _ = range_bounds.swap_remove(*i);
        }

        range_bounds.push(range);
    }

    let valid_id_count: u64 = range_bounds.iter().map(|range| range.1 - range.0 + 1).sum();

    dbg!(valid_id_count);

    Ok(())
}

fn format_input(input: Vec<String>) -> Input {
    let split = input.iter().position(|line| !line.contains("-")).unwrap();

    let ranges: Vec<_> = input[..split]
        .iter()
        .map(|line| match line.split("-").collect::<Vec<_>>()[..] {
            [start, end] => start.parse::<u64>().unwrap()..=end.parse::<u64>().unwrap(),
            _ => panic!(),
        })
        .collect();

    let ids: Vec<_> = input[split..]
        .iter()
        .map(|line| line.parse::<u64>().unwrap())
        .collect();

    (ranges, ids)
}
