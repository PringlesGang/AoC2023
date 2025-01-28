use std::{error::Error, ops::{self, Index}, slice::Iter};

use itertools::Itertools;

#[derive(Debug, Clone)]
struct Grid {
    data: Vec<char>,
    width: usize,
    height: usize,
}

impl Grid {
    fn from(string: &str) -> Grid {
        let height = string.lines().count();
        let width = string.lines().next().map_or(0, str::len);
        let data = string.chars().filter(|char| !char.is_whitespace()).collect();

        Grid{data, width, height}
    }
}

impl Index<usize> for &Grid {
    type Output = [char];

    fn index(&self, index: usize) -> &Self::Output {
        let start = index * self.width;
        let end = start + self.width;
        &self.data[start..end]
    }
}

#[derive(Debug, Clone, Copy)]
struct Coordinate {
    char: usize,
    line: usize,
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


#[derive(Debug, Clone, Copy)]
struct Number {
    coordinate: Coordinate,
    value: u32,
}

impl Number {
    fn length(&self) -> usize {
        if self.value == 0 {
            0
        } else {
            (self.value.ilog10() + 1) as usize
        }
    }

    fn contains(&self, coordinate: Coordinate) -> bool {
        self.coordinate.line == coordinate.line &&
            self.coordinate.char <= coordinate.char &&
            coordinate.char < self.coordinate.char + self.length()
    }

    fn is_mechanical(&self, symbols: &[Coordinate]) -> bool {
        symbols.iter()
            .flat_map(|coordinate| Direction::iterator()
                .filter_map(|direction| coordinate.try_add(*direction))
            )
            .any(|coordinate| self.contains(coordinate))
    }
}

#[derive(Debug, Clone, Copy)]
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
    fn to_offset(self) -> (i8, i8) {
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

    fn iterator() -> Iter<'static, Direction> {
        static DIRECTIONS: [Direction; 8] = [
            Direction::North,
            Direction::NorthEast,
            Direction::East,
            Direction::SouthEast,
            Direction::South,
            Direction::SouthWest,
            Direction::West,
            Direction::NorthWest,
        ];
        DIRECTIONS.iter()
    }
}

fn get_numbers(schematic: &Grid) -> Vec<Number> {
    (0..schematic.height)
        .map(|row| (row, &schematic[row]))
        .flat_map(|(y, line)| line
            .iter()
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

fn get_symbols(schematic: &Grid) -> Vec<Coordinate> {
    (0..schematic.height)
        .flat_map(|line|
            schematic[line]
                .iter()
                .enumerate()
                .filter(|(_, char)| !char.is_numeric() && **char != '.')
                .map(move |(char, _)| Coordinate{char, line})
        )
        .collect()
}

fn get_gears(schematic: &Grid) -> Vec<Coordinate> {
    get_symbols(schematic)
        .into_iter()
        .filter(|Coordinate{char, line}|
            schematic[*line][*char] == '*'
        )
        .collect()
}

fn get_gear_ratio(gear: Coordinate, numbers: &[Number]) -> u32 {
    let adjacent_numbers: Vec<_> = numbers
        .iter()
        .filter(|number| Direction::iterator()
            .filter_map(|direction| gear.try_add(*direction))
            .any(|coordinate| number.contains(coordinate))
        )
        .map(|number| number.value)
        .collect();

    if adjacent_numbers.len() < 2 {
        0
    } else {
        adjacent_numbers.into_iter().product()
    }
}

#[allow(clippy::unnecessary_wraps)]
pub fn solve_1(input: &str) -> Result<String, Box<dyn Error>> {
    let schematic = Grid::from(input);

    let numbers = get_numbers(&schematic);
    let symbols = get_symbols(&schematic);

    let sum: u32 = numbers.into_iter()
        .filter(|number| number.is_mechanical(&symbols))
        .map(|Number{value, ..}| value)
        .sum();

    Ok(format!("The sum of all mechanical numbers is {sum}"))
}

#[allow(clippy::unnecessary_wraps)]
pub fn solve_2(input: &str) -> Result<String, Box<dyn Error>> {
    let schematic = Grid::from(input);

    let numbers = get_numbers(&schematic);
    let sum: u32 = get_gears(&schematic)
        .into_iter()
        .map(|gear| get_gear_ratio(gear, &numbers))
        .sum();

    Ok(format!("The sum of all gear ratios is {sum}"))
}