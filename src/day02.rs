use std::{collections::HashMap, fs, ops::BitAnd};

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Clone, Copy, Hash)]
enum Colour {
    Red,
    Green,
    Blue,
}

#[derive(Debug)]
struct ColouredCube {
    colour: Colour,
    amount: u32,
}

#[derive(Debug)]
struct Hand(HashMap<Colour, u32>);

#[derive(Debug)]
struct Game(Vec<Hand>);

fn read_input() -> Result<String, &'static str> {
    fs::read_to_string("./resources/day02/input.txt")
        .map_err(|_| "Failed to open input file")
}

impl Colour {
    fn parse(string: &str) -> Colour {
        match string {
            "red" => Colour::Red,
            "green" => Colour::Green,
            "blue" => Colour::Blue,
            _ => unreachable!()
        }
    }
}

impl ColouredCube {
    fn parse(string: &str) -> ColouredCube {
        let (amount, colour) = string.split_at(
            string.char_indices()
                .find(|(_, c)| c.is_alphabetic())
                .unwrap()
                .0
        );

        ColouredCube {
            colour: Colour::parse(colour),
            amount: amount.parse().unwrap_or(0)
        }
    }
}

impl Hand {
    fn parse(hand: &str) -> Hand {
        let hash_map: Vec<(Colour, u32)> = hand.split(',')
            .collect::<Vec<&str>>()
            .into_iter()
            .map(ColouredCube::parse)
            .map(|cube| (cube.colour, cube.amount))
            .collect();
            
        Hand(HashMap::from_iter(hash_map))
    }
}

impl Game {
    fn parse(string: &str) -> Game {
        let game = string.split(':')
            .nth(1)
            .unwrap() // How to early-return as None?
            .split(';')
            .map(Hand::parse)
            .collect();
        Game(game)
    }
}

fn parse_input() -> Result<Vec<Game>, &'static str> {
    let input_string = read_input()?;

    let input = input_string.chars() // Remove spaces
        .filter(|c| *c != ' ')
        .collect::<String>()
        .lines() // Split into games
        .map(ToString::to_string)
        .collect::<Vec<String>>()
        .into_iter()
        .map(|string| Game::parse(string.as_str()))
        .collect();

    Ok(input)
}

fn possible_hand(hand: Hand, bag: &Hand) -> bool {
    hand.0.into_iter()
        .map(|(colour, amount)| amount <= bag.0[&colour])
        .reduce(BitAnd::bitand)
        .unwrap_or(true)
}

fn possible_game(game: Game, bag: &Hand) -> bool {
    game.0.into_iter()
        .map(|hand| possible_hand(hand, bag))
        .reduce(BitAnd::bitand)
        .unwrap_or(true)
}

fn get_sum(input: Vec<Game>, hand: &Hand) -> u32 {
    #[allow(clippy::cast_possible_truncation)]
    input.into_iter()
        .enumerate()
        .map(|(index, game)| (index + 1, possible_game(game, hand)))
        .filter(|(_, possible)| *possible)
        .map(|(index, _)| index as u32)
        .sum()
}

pub fn solve_1() {
    println!("Day 02 part 1:");

    let input = match parse_input() {
        Ok(result) => result,
        Err(msg) => {
            println!("Encountered an error: {msg}");
            return;
        },
    };

    let bag = Hand(HashMap::from([
        (Colour::Red, 12),
        (Colour::Green, 13),
        (Colour::Blue, 14),
    ]));

    let sum = get_sum(input, &bag);

    println!("The sum of the indices all valid games is {sum}");

    println!();
}