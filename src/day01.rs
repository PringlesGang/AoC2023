use std::{collections::HashMap, fs, string::ToString};

fn read_input() -> Result<Vec<String>, &'static str> {
    let document= fs::read_to_string("./resources/day01/input.txt")
        .map_err(|_| "Failed to open input file")?;
    Ok(document.lines().map(ToString::to_string).collect())
}

fn get_match(line: &str, productions: &HashMap<String, u32>) -> u32 {
    let string = match productions.keys()
        .collect::<Vec<_>>()
        .into_iter()
        .map(
            |string|
            (
                string,
                line.find(string as &str)
                    .unwrap_or(line.len())
            )
        )
        .reduce(|(string1, loc1), (string2, loc2)| if loc1 <= loc2 { (string1, loc1) } else { (string2, loc2) } ) {
        Some(x) => x.0,
        None => "0"
    };

    *productions.get(string).unwrap_or(&0)
}

fn get_calibration_value(line: &str, productions: &HashMap<String, u32>) -> u32 {
    let first_num = get_match(line, productions);
    let last_num = get_match(line.chars().rev().collect::<String>().as_str(), productions);

    first_num * 10 + last_num
}

fn get_sum(productions: &HashMap<String, u32>) -> Result<u32, &'static str> {
    let lines = read_input()?;

    let sum = lines.iter()
        .map(|line| get_calibration_value(line.as_str(), productions))
        .sum();

    Ok(sum)
}

pub fn solve_1() {
    let productions = (0 .. 10).map(|num: u32| (num.to_string(), num)).collect::<HashMap<_, _>>();

    match get_sum(&productions) {
        Ok(sum) => println!("The sum of all calibration values is {sum}"),
        Err(msg) => eprintln!("{msg}")
    }
}