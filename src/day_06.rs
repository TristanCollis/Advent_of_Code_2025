use common::*;

const EXAMPLE_PATH: &str = r"./day/06/example.txt";
const INPUT_PATH: &str = r"./day/06/input.txt";

pub fn part_1(use_example: bool) -> Result<()> {
    _part_1(format_input(parse_file(if use_example {
        &EXAMPLE_PATH
    } else {
        &INPUT_PATH
    })?))
}

pub fn part_2(use_example: bool) -> Result<()> {
    _part_2(format_part_2(custom_parse(if use_example {
        &EXAMPLE_PATH
    } else {
        &INPUT_PATH
    })?))
}

type Input = (Vec<String>, VecArray<u64>);

#[derive(Debug, Clone)]
struct VecArray<T> {
    pub rows: usize,
    pub cols: usize,
    data: Vec<Vec<T>>,
}

impl<T> VecArray<T> {
    fn new(data: Vec<Vec<T>>) -> Result<Self> {
        let rows = data.len();
        let cols = {
            if rows > 0 {
                data[0].len()
            } else {
                0
            }
        };

        for (i, row) in data.iter().enumerate() {
            if row.len() != cols {
                return Err(format!(
                    "Jagged arrays not supported. Expected {:} columns from first row. found {:} columns in row {:}",
                    cols,
                    row.len(),
                    i
                )
                .into());
            }
        }

        Ok(Self { rows, cols, data })
    }

    fn transpose(&self) -> VecArray<&T> {
        let data: Vec<Vec<&T>> = (0..self.cols)
            .map(|j| (0..self.rows).map(|i| &self.data[i][j]).collect())
            .collect();

        VecArray::new(data).unwrap()
    }

    fn into_transpose(self) -> VecArray<T> {
        let cols = self.cols.clone();
        let mut iters: Vec<_> = self.into_iter().map(|row| row.into_iter()).collect();
        let data: Vec<Vec<T>> = (0..cols)
            .map(|_| iters.iter_mut().map(|elem| elem.next().unwrap()).collect())
            .collect();

        return VecArray::new(data).unwrap();
    }

    fn iter(&'_ self) -> VecArrayIterator<'_, T> {
        VecArrayIterator {
            vecarray: &self,
            index: 0,
        }
    }
}

struct VecArrayIterator<'a, T> {
    vecarray: &'a VecArray<T>,
    index: usize,
}
impl<'a, T> Iterator for VecArrayIterator<'a, T> {
    type Item = &'a Vec<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.vecarray.rows {
            let output = &self.vecarray.data[self.index];
            self.index += 1;
            return Some(output);
        } else {
            return None;
        }
    }
}
impl<T> IntoIterator for VecArray<T> {
    type Item = Vec<T>;

    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        return self.data.into_iter();
    }
}

fn _part_1((operators, operands): Input) -> Result<()> {
    let output: u64 = operands
        .into_transpose()
        .iter()
        .zip(operators)
        .map(|(row, operator)| -> u64 {
            match operator.as_str() {
                "+" => row.iter().sum(),
                "*" => row.iter().product(),
                _ => panic!(),
            }
        })
        .sum();

    dbg!(output);

    Ok(())
}

fn _part_2((operators, operands): (Vec<String>, Vec<Vec<u64>>)) -> Result<()> {
    let output: u64 = operators.iter().zip(operands)
    .map(
        |(operator, row)| -> u64{
            match operator.as_str() {
                "+" => row.iter().sum(),
                "*" => row.iter().product(),
                _ => panic!()
            }
        }
    ).sum();

    dbg!(output);

    Ok(())
}

fn format_input(input: Vec<String>) -> Input {
    let operators: Vec<String> = input[input.len() - 1]
        .split_whitespace()
        .map(|s| s.into())
        .collect();

    let operands = VecArray::new(
        input[..input.len() - 1]
            .iter()
            .map(|line| {
                line.split_whitespace()
                    .map(|s| s.parse::<u64>().unwrap())
                    .collect()
            })
            .collect(),
    )
    .unwrap();

    (operators, operands)
}

fn format_part_2(input: Vec<String>) -> (Vec<String>, Vec<Vec<u64>>) {
    let operators = input.clone()[input.len()-1].split_whitespace().map(|s| s.into()).collect::<Vec<String>>();

    let chars = VecArray::new(
        input[..input.len() - 1]
            .iter()
            .map(|line| line.chars().collect())
            .collect(),
    )
    .unwrap()
    .into_transpose();

    let numbers = chars
        .into_iter()
        .map(|row| {
            row.into_iter().fold(0, |acc, c| match c.to_digit(10) {
                Some(n) => 10 * acc + n as u64,
                None => acc,
            })
        })
        .collect::<Vec<_>>();

    let operands = 
        numbers
            .split(|&n| n == 0)
            .map(|column| column.to_vec())
            .collect::<Vec<_>>();

    (operators, operands)
}

fn custom_parse(path: &dyn AsRef<std::path::Path>) -> Result<Vec<String>> {
    Ok(std::fs::read_to_string(path)?
        .split("\r\n")
        .filter_map(|elem| match elem {
            "" => None,
            s => Some(s.into()),
        })
        .collect())
}
