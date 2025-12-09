use common::*;
use ndarray::{Array2, ArrayBase, Dim, OwnedRepr};

const EXAMPLE_PATH: &str = r"./day/07/example.txt";
const INPUT_PATH: &str = r"./day/07/input.txt";

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

#[derive(Debug, Clone, Copy, PartialEq)]
enum Cell {
    Source,
    Splitter,
    Empty,
}

type Input = ArrayBase<OwnedRepr<Cell>, Dim<[usize; 2]>, Cell>;

fn _part_1(input: Input) -> Result<()> {
    let mut beams = vec![false; input.shape()[1]];
    beams[input
        .row(0)
        .iter()
        .position(|cell| cell == &Cell::Source)
        .unwrap()] = true;

    let mut splits = 0;

    for i in 1..input.shape()[0] {
        let mut next_beams = vec![false; beams.len()];

        for (j, (&beam, &cell)) in beams.iter().zip(input.row(i)).enumerate() {
            if beam && cell == Cell::Splitter {
                splits += 1;

                if 0 < j {
                    next_beams[j - 1] = true;
                }
                if j < beams.len() - 1 {
                    next_beams[j + 1] = true;
                }
            } else {
                next_beams[j] |= beam;
            }
        }
        beams = next_beams;
    }

    dbg!(splits);

    Ok(())
}

fn _part_2(input: Input) -> Result<()> {
    let mut possible_timelines = vec![0; input.shape()[1]];
    possible_timelines[input
        .row(0)
        .iter()
        .position(|&cell| cell == Cell::Source)
        .unwrap()] = 1;

    for i in 1..input.shape()[0] {
        let mut next_timelines = vec![0; possible_timelines.len()];

        for (j, (&timelines, &cell)) in possible_timelines.iter().zip(input.row(i)).enumerate() {
            if cell == Cell::Splitter {
                if 0 < j {
                    next_timelines[j - 1] += timelines;
                }
                if j < next_timelines.len() - 1 {
                    next_timelines[j + 1] += timelines;
                }
            }
            next_timelines[j] += timelines;
        }

        for (j, &cell) in input.row(i).iter().enumerate() {
            if cell == Cell::Splitter {
                next_timelines[j] = 0;
            }
        }

        possible_timelines = next_timelines;
    }
    let total_timelines: u64 = possible_timelines.iter().sum();
    dbg!(total_timelines);

    Ok(())
}

fn format_input(input: Vec<String>) -> Input {
    let arr = Array2::from_shape_vec(
        (input.len() / 2, input[0].len()),
        input
            .into_iter()
            .step_by(2)
            .flat_map(|line| {
                line.chars()
                    .map(|c| match c {
                        'S' => Cell::Source,
                        '^' => Cell::Splitter,
                        '.' => Cell::Empty,
                        _ => panic!(),
                    })
                    .collect::<Vec<_>>()
            })
            .collect(),
    )
    .unwrap();

    arr
}
