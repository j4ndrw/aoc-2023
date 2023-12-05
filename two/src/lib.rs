use std::{error::Error, str::FromStr};

#[derive(Debug, Default)]
struct CubeSet {
    red: usize,
    blue: usize,
    green: usize,
}

#[derive(Debug)]
struct Game {
    id: usize,
    cube_sets: Vec<CubeSet>,
}

impl FromStr for CubeSet {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let elements = s.split(", ");
        let cube_set = elements.fold(
            CubeSet::default(),
            |CubeSet { red, blue, green }, element| {
                let (many, color) = element.split_once(' ').unwrap();
                let many = many.parse::<usize>().unwrap();
                return CubeSet {
                    red: red + ((color == "red") as usize) * many,
                    green: green + ((color == "green") as usize) * many,
                    blue: blue + ((color == "blue") as usize) * many,
                };
            },
        );
        return Ok(cube_set);
    }
}

impl FromStr for Game {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (game_id, cube_sets) = s.split_once(": ").unwrap();
        let game_id = game_id.split(' ').last().unwrap().parse::<usize>()?;
        let cube_sets = cube_sets
            .split("; ")
            .map(|cube_set| cube_set.parse::<CubeSet>().unwrap())
            .collect::<Vec<_>>();

        return Ok(Game {
            id: game_id,
            cube_sets,
        });
    }
}

pub fn first_solution(input: &str) -> usize {
    const MAX_CUBE_SET: CubeSet = CubeSet {
        red: 12,
        green: 13,
        blue: 14,
    };
    return input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.parse::<Game>().unwrap())
        .filter(|game| {
            let impossible = game.cube_sets.iter().any(|cube_set| {
                cube_set.red > MAX_CUBE_SET.red
                    || cube_set.blue > MAX_CUBE_SET.blue
                    || cube_set.green > MAX_CUBE_SET.green
            });
            return !impossible;
        })
        .map(|game| game.id)
        .sum();
}

pub fn second_solution(input: &str) -> usize {
    return input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.parse::<Game>().unwrap())
        .map(|game| {
            return game.cube_sets.iter().fold((0, 0, 0), |acc, cube_set| {
                let (min_red, min_green, min_blue) = acc;
                return (
                    cube_set.red.max(min_red),
                    cube_set.green.max(min_green),
                    cube_set.blue.max(min_blue),
                );
            });
        })
        .map(|(red, green, blue)| red * green * blue)
        .sum();
}
