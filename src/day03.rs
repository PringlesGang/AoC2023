use std::{error::Error, ops};

use itertools::Itertools;

#[derive(Debug, Clone, Copy)]
struct Coordinate {
    char: usize,
    line: usize,
}

#[derive(Debug, Clone, Copy)]
struct Number {
    coordinate: Coordinate,
    value: u32,
}

#[derive(Debug)]
enum Direction {
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
}

impl Direction {
    fn to_offset(&self) -> (i8, i8) {
        match self {
            Direction::North => (0, 1),
            Direction::NorthEast => (1, 1),
            Direction::East => (1, 0),
            Direction::SouthEast => (1, -1),
            Direction::South => (0, -1),
            Direction::SouthWest => (-1, -1),
            Direction::West => (-1, 0),
            Direction::NorthWest => (-1, 1),
        }
    }
}

impl ops::Add<Coordinate> for Coordinate {
    type Output = Coordinate;

    fn add(self, rhs: Coordinate) -> Coordinate {
        Coordinate {
            char: self.char + rhs.char,
            line: self.line + rhs.line,
        }
    }
}

impl Coordinate {
    #[allow(clippy::needless_pass_by_value)]
    fn try_add(self, direction: Direction) -> Option<Coordinate> {
        let offset = direction.to_offset();
        let char = self.char.checked_add_signed(offset.0 as isize)?;
        let line = self.line.checked_add_signed(offset.1 as isize)?;
        Some(Coordinate{char, line})
    }
}

impl Number {
    fn length(&self) -> usize {
        if self.value == 0 {
            0
        } else {
            (self.value.ilog10() + 1) as usize
        }
    }

    fn is_mechanical(&self, input: &str) -> bool {
        let length = self.length();
        let height = input.lines().count();
        let width = input.lines().next().map_or(0, str::len);
        
        let start = self.coordinate;
        let end = self.coordinate + Coordinate{char: length - 1, line: 0};

        let boundary_ends = Vec::from_iter([
            start.try_add(Direction::NorthWest),
            start.try_add(Direction::West),
            start.try_add(Direction::SouthWest),
            end.try_add(Direction::NorthEast),
            end.try_add(Direction::East),
            end.try_add(Direction::SouthEast),
        ]);
        let boundary_middle: Vec<_> = (0..length)
            .map(|char| start + Coordinate{char, line: 0})
            .flat_map(|coord| [coord.try_add(Direction::North), coord.try_add(Direction::South)])
            .collect();
        let boundary: Vec<_> = boundary_ends.into_iter()
            .chain(boundary_middle)
            .flatten()
            .filter(|coord| coord.char < width && coord.line < height)
            .collect();

        boundary.into_iter()
            .map(|coord| {
                let line = input.lines().nth(coord.line)?;
                let char = line.chars().nth(coord.char)?;
                Some(char)
            })
            .collect::<Option<Vec<_>>>()
            .unwrap()
            .into_iter()
            .any(|char| !char.is_numeric() && char != '.')
    }
}

fn get_numbers(input: &str) -> Vec<Number> {
    input.lines()
        .enumerate()
        .flat_map(|(y, line)| line
            .chars()
            .enumerate()
            .filter_map(|(x, char)| char
                .to_digit(10)
                .map(|char| (x, x, char))
            )
            .coalesce(|(start1, end1, v1), (start2, end2, v2)| {
                if start2 == end1 + 1 {
                    Ok((start1, end2, v1 * 10 + v2))
                } else {
                    Err(((start1, end1, v1), (start2, end2, v2)))
                }
            })
            .map(move |(x, _, v)|
                Number{
                    coordinate: Coordinate{char: x, line: y},
                    value: v
                }
            )
        )
        .collect()
}

#[allow(clippy::unnecessary_wraps)]
pub fn solve_1(input: &str) -> Result<String, Box<dyn Error>> {
    let mechanical_numbers: Vec<_> = get_numbers(input)
        .into_iter()
        .filter(|number| number.is_mechanical(input))
        .collect();

    let sum: u32 = mechanical_numbers.into_iter()
        .map(|number| number.value)
        .sum();

    Ok(format!("The sum of all mechanical numbers is {sum}"))
}