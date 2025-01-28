use std::{error::Error, fmt::Display, fs};

mod day01;
mod day02;
mod day03;

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
enum Part {
    One = 1,
    Two = 2,
}

impl Display for Part {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let string = match self {
            Part::One => "part 1",
            Part::Two => "part 2",
        };

        write!(f, "{string}")
    }
}

impl TryFrom<u8> for Part {
    type Error = String;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Part::One),
            2 => Ok(Part::Two),
            _ => Err(format!("Failed to convert {value} to Part!")),
        }
    }
}

struct Problem<'a, F> where
    F: Fn(&str) -> Result<String, Box<dyn Error>>
{
    solver: F,
    day: u32,
    part: Part,
    input_filename: &'a str,
}

impl<F> Problem<'_, F> where
    F: Fn(&str) -> Result<String, Box<dyn Error>>
{
    fn solve(self) {
        println!("Solving day {} {}...", self.day, self.part);

        let input_path = format!("./resources/day{:02}/{}", self.day, self.input_filename);

        let input = match fs::read_to_string(input_path) {
            Ok(input) => input,
            Err(msg) => {
                eprintln!("Failed to read input file: {msg}");
                return;
            }
        };

        match (self.solver)(input.as_str()) {
            Ok(result) => println!("{result}"),
            Err(msg) => eprintln!("{msg}"),
        }

        println!();
    }
}


fn solve_all() {
    let solvers = [
        day01::solve_1,
        day01::solve_2,
        day02::solve_1,
        day02::solve_2,
        day03::solve_1,
        day03::solve_2,
    ];
    
    #[allow(clippy::cast_possible_truncation)]
    let problems = solvers.into_iter()
        .enumerate()
        .map(|(index, solver)|
            Problem {
                solver,
                day: (index / 2 + 1) as u32,
                part: Part::try_from((index % 2 + 1) as u8).unwrap(),
                input_filename: "input.txt",
            }
        );
    
    problems.for_each(Problem::solve);
}

fn main() {
    println!("Solving all 2023 days...\n");
    
    solve_all();
}
