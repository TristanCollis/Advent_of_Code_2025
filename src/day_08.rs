use std::{
    collections::HashSet,
    ops::{Add, Mul, Sub},
};

use common::*;
use itertools::iproduct;
use ndarray::Zip;
use num_traits::Zero;

const EXAMPLE_PATH: &str = r"./day/08/example.txt";
const INPUT_PATH: &str = r"C:./day/08/input.txt";

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

type Input = Vec<([i64; 3], [i64; 3])>;

fn _part_1(mut input: Input) -> Result<()> {
    let mut circuits: Vec<HashSet<[i64; 3]>> = vec![];

    for _ in 0..1000 {
        let (a, b) = input.pop().unwrap();

        let circuit_a = get_circuit(a, &mut circuits);
        let circuit_b = get_circuit(b, &mut circuits);

        circuits.push(HashSet::from_iter(
            circuit_a.into_iter().chain(circuit_b.into_iter()),
        ));
    }

    circuits.sort_by_key(|circuit| circuit.len());

    dbg!(&circuits[circuits.len() - 3..]
        .iter()
        .map(|circuit| circuit.len())
        .product::<usize>());

    Ok(())
}

fn _part_2(mut input: Input) -> Result<()> {
    let mut last_connected_pair: ([i64; 3], [i64; 3]) = input[2];

    let mut circuits: Vec<HashSet<[i64; 3]>> = vec![];
    while let Some((a, b)) = input.pop() {

        let circuit_a = get_circuit(a.clone(), &mut circuits);
        if circuit_a.contains(&b) {
            circuits.push(circuit_a);
            continue;
        }

        let circuit_b = get_circuit(b.clone(), &mut circuits);

        circuits.push(HashSet::from_iter(
            circuit_a.into_iter().chain(circuit_b.into_iter()),
        ));

        last_connected_pair = (a, b);
    }

    dbg!(last_connected_pair.0[0] * last_connected_pair.1[0]);

    Ok(())
}

fn format_input(input: Vec<String>) -> Input {
    let points: Vec<[i64; 3]> = input
        .into_iter()
        .map(|line| match line.split(",").collect::<Vec<_>>()[..] {
            [x, y, z] => [x.parse().unwrap(), y.parse().unwrap(), z.parse().unwrap()],
            _ => panic!(),
        })
        .collect::<Vec<_>>();

    {
        let mut pairs: Vec<([i64; 3], [i64; 3])> = iproduct!(points.clone(), points)
            .filter(|(a, b)| a != b)
            .step_by(2)
            .collect();

        pairs.sort_by_key(|(a, b)| -distance_squared(a, b));
        pairs
    }
}

fn distance_squared<T: Mul<Output = T> + Add + Sub<Output = T> + Copy + Zero>(
    a: &[T; 3],
    b: &[T; 3],
) -> T {
    Zip::from(a)
        .and(b)
        .map_collect(|&a_elem, &b_elem| {
            let diff = a_elem - b_elem;
            diff * diff
        })
        .sum()
}

fn get_circuit(point: [i64; 3], circuits: &mut Vec<HashSet<[i64; 3]>>) -> HashSet<[i64; 3]> {
    for i in 0..circuits.len() {
        if circuits[i].contains(&point) {
            let mut circuit = circuits.swap_remove(i);
            circuit.insert(point);
            return circuit;
        }
    }

    let mut circuit = HashSet::new();
    circuit.insert(point);
    return circuit;
}
