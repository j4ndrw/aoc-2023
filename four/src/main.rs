use std::error::Error;

use one::{first_solution, second_solution};

fn main() -> Result<(), Box<dyn Error>> {
    let part1_sample_output = first_solution(include_str!("../data/part1-sample.txt"));
    println!("part1-sample: {:#?}", part1_sample_output);

    let part1_real_output = first_solution(include_str!("../data/part1-real.txt"));
    println!("part1-real: {:#?}", part1_real_output);

    let part2_sample_output = second_solution(include_str!("../data/part2-sample.txt"));
    println!("part2-sample: {:#?}", part2_sample_output);

    // let part2_real_output = second_solution(include_str!("../data/part2-real.txt"));
    // println!("part2-real: {:#?}", part2_real_output);

    return Ok(());
}
