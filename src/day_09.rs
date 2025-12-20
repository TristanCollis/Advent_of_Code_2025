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
    Corner(Point, TurnDirection)
}

#[derive(Debug, Clone, Copy)]
enum OutDirection {
    North,
    East,
    South,
    West
}

#[derive(Debug, Clone, Copy)]
enum TurnDirection {
    Clockwise,
    Counterclockwise
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


fn enum_walls(points: &Vec<Point>) -> Vec<Wall> {

    let mut walls
    
    for triplet in points.iter().circular_tuple_windows::<(_, _, _)>() {

            #
        #   #
    }

    todo!()
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
