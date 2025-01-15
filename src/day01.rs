use std::{collections::HashMap, error::Error, string::ToString};

fn get_match(line: &str, productions: &HashMap<&str, u32>, reverse: bool) -> u32 {
    let directed_line = if reverse {
        line.chars().rev().collect()
    } else {
        line.to_string()
    };

    let string = productions.keys()
        .collect::<Vec<_>>()
        .into_iter()
        .map(|string| {
            let directed_string = if reverse {
                string.chars().rev().collect()
            } else {
                (*string).to_string()
            };

            (
                string,
                directed_line
                    .find(&directed_string)
                    .unwrap_or(line.len())
            )
        })
        .reduce(|(string1, loc1), (string2, loc2)|
            if loc1 <= loc2 { (string1, loc1) } else { (string2, loc2) }
        ).map_or("0", |(string, _)| string);

    *productions.get(string).unwrap_or(&0)
}

fn get_calibration_value(line: &str, productions: &HashMap<&str, u32>) -> u32 {
    let first_num = get_match(line, productions, false);
    let last_num = get_match(line, productions, true);

    first_num * 10 + last_num
}

fn get_sum(input: &str, productions: &HashMap<&str, u32>) -> u32 {
    input.lines()
        .map(|line| get_calibration_value(line, productions))
        .sum()
}

#[allow(clippy::unnecessary_wraps)]
pub fn solve_1(input: &str) -> Result<String, Box<dyn Error>> {
    let num_strings = (0 .. 10).map(|num| num.to_string()).collect::<Vec<_>>();

    #[allow(clippy::cast_possible_truncation)]
    let productions: HashMap<&str, u32> = num_strings.iter()
        .enumerate()
        .map(|(i, string)| (string.as_str(), i as u32))
        .collect();

    let sum = get_sum(input, &productions);

    Ok(format!("The sum of all calibration values is {sum}"))
}

#[allow(clippy::unnecessary_wraps)]
pub fn solve_2(input: &str) -> Result<String, Box<dyn Error>> {
    let num_strings = (0 .. 10).map(|num| num.to_string()).collect::<Vec<_>>();

    #[allow(clippy::cast_possible_truncation)]
    let productions: HashMap<&str, u32> = num_strings.iter()
        .enumerate()
        .map(|(i, string)| (string.as_str(), i as u32))
        .chain([
            ("one", 1),
            ("two", 2),
            ("three", 3),
            ("four", 4),
            ("five", 5),
            ("six", 6),
            ("seven", 7),
            ("eight", 8),
            ("nine", 9)
        ]).collect();

    let sum = get_sum(input, &productions);

    Ok(format!("The sum of all calibration values is {sum}"))
}