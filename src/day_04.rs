use std::collections::HashSet;

use common::*;

const EXAMPLE_PATH: &str = r"./day/04/example.txt";
const INPUT_PATH: &str = r"./day/04/input.txt";

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

type Input = Vec<Vec<bool>>;

fn _part_1(input: Input) -> Result<()> {
    let neighbor_arr = neighbors(&input);

    let accessible_rolls: usize = input
        .iter()
        .zip(neighbor_arr)
        .map(|(i_row, n_row)| {
            i_row
                .iter()
                .zip(n_row)
                .filter(|(roll_present, neighbors)| **roll_present && *neighbors <= 4)
                .count()
        })
        .sum();

    dbg!(accessible_rolls);

    Ok(())
}

fn _part_2(mut input: Input) -> Result<()> {
    let mut neighbor_count = neighbors(&input);
    let mut removed_rolls = 0;
    let mut rolls_to_remove: HashSet<(usize, usize)> = HashSet::new();

    loop {
        for (i, (i_row, n_row)) in input.iter().zip(&neighbor_count).enumerate() {
            for (j, (i_cell, n_cell)) in i_row.iter().zip(n_row).enumerate() {
                if *i_cell && *n_cell <= 4 {
                    rolls_to_remove.insert((i, j));
                    removed_rolls += 1;
                }
            }
        }
        
        for (i, j) in &rolls_to_remove {
            input[*i][*j] = false;
            for h in (i.max(&1) - 1)..=(i+1).min(input.len()-1) {
                for k in (j.max(&1) - 1)..=(j+1).min(input[0].len()-1) {
                    neighbor_count[h][k] -= 1;
                }
            }
        }

        if rolls_to_remove.is_empty() {
            break;
        }

        rolls_to_remove.clear();
    }
        
    dbg!(removed_rolls);
    

    Ok(())
}

fn format_input(input: Vec<String>) -> Input {
    input
        .into_iter()
        .map(|row| {
            row.chars()
                .map(|cell| match cell {
                    '@' => true,
                    _ => false,
                })
                .collect()
        })
        .collect()
}

fn neighbors(input: &Input) -> Vec<Vec<u8>> {
    let mut neighbor_array: Vec<Vec<u8>> = vec![vec![0; input[0].len()]; input.len()];

    for i in 0..input.len() {
        for j in 0..input[0].len() {
            for h in ((i as i64 - 1).max(0) as usize)..(i + 2).min(input.len()) {
                for k in ((j as i64 - 1).max(0) as usize)..(j + 2).min(input[0].len()) {
                    neighbor_array[i][j] += input[h][k] as u8;
                }
            }
        }
    }

    neighbor_array
}
