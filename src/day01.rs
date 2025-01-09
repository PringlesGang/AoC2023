use std::{fs, string::ToString};

fn read_input() -> Result<Vec<String>, &'static str> {
    let document= fs::read_to_string("./resources/day01/input.txt")
        .map_err(|_| "Failed to open input file")?;
    Ok(document.lines().map(ToString::to_string).collect())
}

fn get_calibration_value(line: &str) -> u32 {
    let numbers = 0 .. 10;
    let number_chars: Vec<char> = numbers.filter_map(|num| char::from_digit(num, 10)).collect();

    let first_num : u32 = line.chars()
        .find(|character| number_chars.contains(character))
        .and_then(|character| character.to_digit(10))
        .unwrap_or(0);
    let last_num : u32 = line.chars()
        .rev()
        .find(|character| number_chars.contains(character))
        .and_then(|character| character.to_digit(10))
        .unwrap_or(0);

    first_num * 10 + last_num
}

fn get_sum() -> Result<u32, &'static str> {
    let lines = read_input()?;

    let sum = lines.iter()
        .map(|line| get_calibration_value(line.as_str()))
        .sum();

    Ok(sum)
}

pub fn solve() {
    match get_sum() {
        Ok(sum) => println!("The sum of all calibration values is {sum}"),
        Err(msg) => eprintln!("{msg}")
    }
}