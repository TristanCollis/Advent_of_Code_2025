use std::collections::HashMap;

use common::*;
use itertools::Itertools;

const EXAMPLE_PATH: &str = r"./day/11/example.txt";
const INPUT_PATH: &str = r"./day/11/input.txt";

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

type Input = HashMap<String, Vec<String>>;

fn _part_1(input: Input) -> Result<()> {
    let nodes = input;
    let mut stack: Vec<String> = vec!["you".into()];

    let mut count = 1;

    while let Some(node) = stack.pop() {
        if &node == "out" {
            continue;
        } else {
            let outs = nodes.get(&node).unwrap();
            count += outs.len() - 1;
            for next_node in outs {
                stack.push(next_node.clone());
            }
        }
    }

    dbg!(count);

    Ok(())
}

fn _part_2(input: Input) -> Result<()> {
    #[derive(Clone, Copy)]
    enum Visited {
        Neither,
        DAC,
        FFT,
        Both,
    }

    struct State {
        node: String,
        visited: Visited,
    }

    let nodes = input;
    let mut stack: Vec<State> = vec![State {
        node: "svr".into(),
        visited: Visited::Neither,
    }];

    let mut count = 1;

    while let Some(state) = stack.pop() {
        if &state.node == "out" {
            match state.visited {
                Visited::Both => count += 1,
                _ => {}
            }

            continue;
        }

        for node in nodes.get(&state.node).unwrap() {
            stack.push(State {
                node: node.clone(),
                visited: state.visited,
            })
        }
    }

    dbg!(count);

    Ok(())
}

fn format_input(input: Vec<String>) -> Input {
    let nodes: HashMap<String, Vec<String>> = HashMap::from_iter(input.iter().map(|line| {
        let (name, outs) = line.split(":").collect_tuple().unwrap();
        (
            name.replace(":", ""),
            outs.split_whitespace().map_into::<String>().collect_vec(),
        )
    }));

    nodes
}
