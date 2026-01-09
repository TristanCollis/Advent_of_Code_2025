use std::collections::HashSet;

use common::*;
use itertools::Itertools;

const EXAMPLE_PATH: &str = r"./day/09/example.txt";
const INPUT_PATH: &str = r"./day/09/input.txt";

type Int = i64;
type Point = (Int, Int);
type Input = Vec<Point>;

#[derive(Debug, Clone, Copy)]
enum Wall {
    Edge(Point, OutDirection),
    Corner(Point, TurnDirection),
}

#[derive(Debug, Clone, Copy)]
enum OutDirection {
    North,
    East,
    South,
    West,
}

#[derive(Debug, Clone, Copy)]
enum TurnDirection {
    Clockwise,
    Counterclockwise,
}

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

fn _part_1(input: Input) -> Result<()> {
    let area = input
        .iter()
        .combinations(2)
        .map(|pair| area(pair[0], pair[1]))
        .max();

    dbg!(area);

    Ok(())
}

fn _part_2(input: Input) -> Result<()> {
    let walls = input
    .iter()
    .circular_tuple_windows()
    .map(
        |(a, b)| {
            wall_segments(*a, *b)
        }
    )
    .fold(
        HashSet::new(),
        |acc, elem| {
            &acc | &elem
        }
    );

    let mut max_area = 0;

    for (a, b) in input.iter().combinations(2).map(|pair| (pair[0], pair[1])) {
        if area(a, b) <= max_area{
            continue;
        }

        let (tr, bl) = ordered_corners(*a, *b);
        

    }

    Ok(())
}

fn format_input(input: Vec<String>) -> Input {
    input
        .into_iter()
        .map(|line| match line.split(",").collect::<Vec<_>>()[..] {
            [a, b] => (a.parse().unwrap(), b.parse().unwrap()),
            _ => panic!(),
        })
        .collect()
}

fn area(a: &Point, b: &Point) -> Int {
    (a.0 - b.0 + 1).abs() * (a.1 - b.1 + 1).abs()
}

fn encloses(a: &Point, b: &Point, point: &Point) -> bool {
    let (topleft, botright) = ordered_corners(*a, *b);
    topleft.0 < point.0 && point.0 < botright.0 && topleft.1 < point.1 && point.1 < botright.1
}

fn wall_segments(a: Point, b: Point) -> HashSet<Point> {
    let (topleft, botright) = ordered_corners(a, b);

    let mut points: HashSet<Point> = HashSet::new();
    for i in topleft.0..=botright.0 {
        points.insert((i, topleft.1));
        points.insert((i, botright.1));
    }
    for j in topleft.1..=botright.1 {
        points.insert((topleft.0, j));
        points.insert((botright.0, j));
    }

    points
}

fn ordered_corners(a: Point, b: Point) -> (Point, Point) {
    ((a.0.min(b.0), a.1.min(b.1)), (a.0.max(b.0), a.1.max(b.1)))
}

fn exterior_border(walls: &HashSet<Point>) -> HashSet<Point> {
    let top = walls.iter().map(|p| p.0).min().unwrap() - 1;
    let bottom = walls.iter().map(|p| p.0).max().unwrap() + 1;
    let left = walls.iter().map(|p| p.1).min().unwrap() - 1;
    let right = walls.iter().map(|p| p.1).max().unwrap() + 1;

    dbg!(top);
    dbg!(bottom);
    dbg!(left);
    dbg!(right);

    let mut layer_0: HashSet<Point>;
    let mut layer_1: HashSet<Point> = HashSet::new();
    let mut layer_2: HashSet<Point> = HashSet::new();
    let mut border: HashSet<Point> = HashSet::new();

    layer_1.insert((top, left));

    layer_2.insert((top + 1, left));
    layer_2.insert((top, left + 1));

    while layer_2.len() > 0 {
        layer_0 = layer_1;
        layer_1 = HashSet::from_iter(layer_2.drain());

        for point in layer_1.iter() {
            for offset in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                let new_point = (point.0 + offset.0, point.1 + offset.1);

                if !(top <= new_point.0
                    && new_point.0 <= bottom
                    && left <= new_point.1
                    && new_point.1 <= right)
                {
                    continue;
                }

                if walls.contains(&new_point) {
                    border.insert(*point);
                    continue;
                }

                if layer_0.contains(&new_point) || layer_1.contains(&new_point) {
                    continue;
                }

                layer_2.insert(new_point);
            }
        }
    }

    border
}

fn show_grid(points: &HashSet<Point>) {
    let mut screen = vec![
        vec!["."; 1 + points.iter().map(|p| p.1).max().unwrap() as usize];
        1 + points.iter().map(|p| p.0).max().unwrap() as usize
    ];

    for (i, j) in points {
        screen[*i as usize][*j as usize] = "#";
    }

    let text = screen.into_iter().map(|line| line.join(" ")).join("\n");

    println!("{text}");
}
