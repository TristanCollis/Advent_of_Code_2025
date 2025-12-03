use common::*;

const EXAMPLE_PATH: &str = r"./day/01/example.txt";
const INPUT_PATH: &str = r"./day/01/input.txt";

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

type Input = Vec<i16>;
enum Sign {
    Positive,
    Negative,
    Zero,
}

fn _part_1(input: Input) -> Result<()> {
    let mut readout = 50;
    let password = input
        .into_iter()
        .filter_map(|turn| {
            readout = (readout + turn) % 100;
            if readout == 0 {
                Some(())
            } else {
                None
            }
        })
        .count();

    dbg!(password);
    Ok(())
}

fn _part_2(input: Input) -> Result<()> {
    let mut readout: i32 = 50;
    let mut count: i32 = 0;
    let mut sign = Sign::Positive;

    for turn in input {
        readout += turn as i32;

        match (sign, signum(readout)) {
            (Sign::Positive, Sign::Positive)
            | (Sign::Negative, Sign::Negative)
            | (Sign::Zero, _) => count += (readout / 100).abs(),

            (_, Sign::Zero) => count += 1,

            _ => count += 1 + (readout / 100).abs(),
        }

        readout %= 100;
        sign = signum(readout);
    }
    
    dbg!(count);

    Ok(())
}

fn format_input(input: Vec<String>) -> Input {
    input
        .iter()
        .map(|line| {
            line[1..].parse::<i16>().unwrap() * {
                if line.starts_with("L") {
                    -1
                } else {
                    1
                }
            }
        })
        .collect::<Vec<_>>()
}

fn signum(n: i32) -> Sign {
    if n > 0 {
        return Sign::Positive;
    }
    if n < 0 {
        return Sign::Negative;
    }
    Sign::Zero
}
