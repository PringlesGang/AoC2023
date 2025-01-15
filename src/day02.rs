use std::{collections::HashMap, error::Error};

use itertools::Itertools;

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
    fn parse(string: &str) -> Result<ColouredCube, String> {
        let colour_start = string.char_indices()
                .find(|(_, c)| c.is_alphabetic())
                .ok_or_else(|| format!("Failed to parse colour: {string}"))?
                .0;
        let (amount, colour) = string.split_at(colour_start);

        Ok(ColouredCube {
            colour: Colour::parse(colour),
            amount: amount.parse().unwrap_or(0)
        })
    }
}

impl Hand {
    fn parse(hand: &str) -> Result<Self, String> {
        let hash_map: HashMap<Colour, u32> = hand.split(',')
            .map(ColouredCube::parse)
            .map_ok(|cube| (cube.colour, cube.amount))
            .collect::<Result<_, _>>()?;
            
        Ok(Self(hash_map))
    }

    fn power(&self) -> u32 {
        self.0.values().product()
    }
}

impl Game {
    fn parse(string: &str) -> Result<Self, String> {
        let game = string.split_once(':')
            .ok_or_else(|| format!("Game did not contain ':': {string}"))?
            .1
            .split(';')
            .map(Hand::parse)
            .collect::<Result<_, _>>()?;

        Ok(Self(game))
    }
}

fn parse_input(input: &str) -> Result<Vec<Game>, String> {
    input.chars() // Remove spaces
        .filter(|c| *c != ' ')
        .collect::<String>()
        .lines() // Split into games
        .map(Game::parse)
        .collect()
}

fn possible_hand(hand: &Hand, bag: &Hand) -> bool {
    hand.0.iter().all(|(colour, amount)| amount <= &bag.0[colour])
}

fn possible_game(game: &Game, bag: &Hand) -> bool {
    game.0.iter().all(|hand| possible_hand(hand, bag))
}

fn minimum_hand_necessary(game: &Game) -> Hand {
    todo!()
}

fn get_sum<'a>(input: impl IntoIterator<Item = &'a Game>, hand: &Hand) -> u32 {
    #[allow(clippy::cast_possible_truncation)]
    input.into_iter()
        .enumerate()
        .map(|(index, game)| (index + 1, possible_game(game, hand)))
        .filter_map(|(index, possible)| possible.then_some(index as u32))
        .sum()
}

pub fn solve_1(input: &str) -> Result<String, Box<dyn Error>> {
    let bag = Hand(HashMap::from([
        (Colour::Red, 12),
        (Colour::Green, 13),
        (Colour::Blue, 14),
    ]));

    let games = parse_input(input)?;
    let sum = get_sum(&games, &bag);

    Ok(format!("The sum of the indices all valid games is {sum}"))
}

pub fn solve_2(input: &str) -> Result<String, Box<dyn Error>> {
    let games = parse_input(input)?;

    Ok(format!("The result is who the fuck even knows??"))
}