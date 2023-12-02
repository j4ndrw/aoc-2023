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

impl FromStr for Game {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (game_id, cube_sets) = s.split_once(": ").unwrap();

        let game_id = game_id.split(' ').last().unwrap().parse::<usize>()?;
        let cube_sets = cube_sets
            .split("; ")
            .map(|cube_set| {
                let elements = cube_set.split(", ");
                let mut cube_set = CubeSet::default();
                for element in elements {
                    let (many, color) = element.split_once(' ').unwrap();
                    let many = many.parse::<usize>().unwrap();
                    match color {
                        "blue" => cube_set.blue += many,
                        "green" => cube_set.green += many,
                        "red" => cube_set.red += many,
                        _ => {}
                    }
                }
                return cube_set;
            })
            .collect::<Vec<_>>();

        return Ok(Game {
            id: game_id,
            cube_sets,
        });
    }
}

const MAX_CUBE_SET: CubeSet = CubeSet {
    red: 12,
    green: 13,
    blue: 14,
};

fn first_solution(input: &str) -> usize {
    return input
        .split('\n')
        .filter(|line| !line.is_empty())
        .map(|line| line.parse::<Game>().unwrap())
        .fold(0, |acc, game| {
            let impossible = game.cube_sets.iter().any(|cube_set| {
                cube_set.red > MAX_CUBE_SET.red
                    || cube_set.blue > MAX_CUBE_SET.blue
                    || cube_set.green > MAX_CUBE_SET.green
            });
            if impossible {
                return acc;
            }
            return acc + game.id;
        });
}

fn second_solution(input: &str) -> usize {
    0
}

fn main() -> Result<(), Box<dyn Error>> {
    let part1_sample_output = first_solution(include_str!("../part1-sample.txt"));
    let part1_real_output = first_solution(include_str!("../part1-real.txt"));

    // let part2_sample_output = second_solution(include_str!("../part2-sample.txt"));
    // let part2_real_output = second_solution(include_str!("../part2-real.txt"));

    println!("part1-sample: {:#?}", part1_sample_output);
    println!("part1-real: {:#?}", part1_real_output);

    // println!("part2-sample: {:#?}", part2_sample_output);
    // println!("part2-real: {:#?}", part2_real_output);

    return Ok(());
}
