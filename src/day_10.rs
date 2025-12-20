use common::*;
use itertools::Itertools;
use ndarray::{concatenate, s, Array2, Axis};
use num_traits::PrimInt;
use regex::Regex;

const EXAMPLE_PATH: &str = r"./day/10/example.txt";
const INPUT_PATH: &str = r"./day/10/input.txt";

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

type Input = Vec<Schematic>;
type Int = u16;
type Light = Int;
type Buttons = Vec<Int>;
type JoltageReqs = Vec<Int>;
type Schematic = (Light, Buttons, JoltageReqs);

fn _part_1(input: Input) -> Result<()> {
    let total_presses: u64 = input
        .into_iter()
        .map(|(light, buttons, _)| {
            (0..(1 << buttons.len()))
                .filter_map(|button_mask: Int| {
                    let output = buttons
                        .iter()
                        .enumerate()
                        .fold(0 as Int, |acc, (j, button)| {
                            acc ^ ((0 as Int).wrapping_sub(((button_mask as Int >> j) & 1) as Int)
                                & button)
                        });

                    if output != light {
                        return None;
                    }

                    let presses: Int = (0..buttons.len()).map(|j| (button_mask >> j) & 1).sum();

                    Some(presses)
                })
                .min()
                .unwrap() as u64
        })
        .sum();

    dbg!(total_presses);

    Ok(())
}

fn _part_2(input: Input) -> Result<()> {
    let (_, buttons, _) = &input[0];

    let V: Array2<f64> = ndarray::arr2(&[
        [3.0, 7.0],
        [1.0, -4.0],
    ]);
    dbg!(det(&V));

    Ok(())
}

fn format_input(input: Vec<String>) -> Vec<Schematic> {
    let light_re = Regex::new(r"\[(.*?)\]").unwrap();
    let button_re = Regex::new(r"\((.*?)\)").unwrap();
    let joltage_re = Regex::new(r"\{(.*?)\}").unwrap();

    input
        .into_iter()
        .map(|line| {
            let light_match = light_re.captures(&line).unwrap().extract::<1>().1[0];
            let light = light_match
                .chars()
                .rev()
                .map(|c| c == '#')
                .fold(0, |acc, elem| (acc << 1) | elem as Int);

            let button_matches = button_re.captures_iter(&line);
            let buttons = button_matches
                .map(|elem| {
                    elem.extract::<1>().1[0]
                        .split(",")
                        .map(|c| c.parse::<Int>().unwrap())
                        .fold(0 as Int, |acc, elem| acc | (1 << elem))
                })
                .collect_vec();

            let joltage_match = joltage_re.captures(&line).unwrap().extract::<1>().1[0];
            let joltages = joltage_match
                .split(",")
                .map(|c| c.parse::<Int>().unwrap())
                .collect_vec();

            (light, buttons, joltages)
        })
        .collect_vec()
}

fn int_to_map(button: u16, length: usize) -> Vec<u16> {
    (0..length).map(|bit| (button >> bit) & 1).collect()
}

fn zip_add<T: PrimInt>(v1: Vec<impl Into<T>>, v2: Vec<impl Into<T>>) -> Vec<T> {
    return v1
        .into_iter()
        .zip(v2.into_iter())
        .map(|(a, b)| a.into() + b.into())
        .collect_vec();
}

fn det(matrix: &Array2<f64>) -> Option<f64> {
    if matrix.shape()[..] == [1, 1] {
        return matrix.get((0, 0)).copied();
    }

    Some(
        matrix
            .row(0)
            .iter()
            .enumerate()
            .filter_map(|(i, &elem)| {
                Some(
                    elem * (-1).pow(i as u32) as f64
                        * det(&concatenate![
                            Axis(1),
                            matrix.slice(s![1.., 0..i]),
                            matrix.slice(s![1.., i + 1..])
                        ])?,
                )
            })
            .sum(),
    )
}
