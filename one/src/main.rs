use std::error::Error;

trait Calibratable {
    fn word_to_int(&self) -> String;
    fn get_calibration_value(&self) -> i32;
}

impl Calibratable for str {
    fn word_to_int(&self) -> String {
        let mut buf = String::new();
        let mut new_str = String::new();

        for c in self.chars() {
            buf.push(c);
            let matched_str = match buf.as_str() {
                "zero" | "twenty" | "thirty" | "forty" | "fifty" | "sixty" | "seventy"
                | "eighty" | "ninety" => Some("0"),
                "one" => Some("1"),
                "two" => Some("2"),
                "three" => Some("3"),
                "four" => Some("4"),
                "five" => Some("5"),
                "six" => Some("6"),
                "seven" => Some("7"),
                "eight" => Some("8"),
                "nine" => Some("9"),
                _ => None,
            };
            if let Some(digit) = matched_str {
                buf.clear();
                new_str.push_str(digit);
            } else if c.is_digit(10) {
                buf.clear();
                new_str.push(c);
            }
        }
        return new_str;
    }

    fn get_calibration_value(&self) -> i32 {
        self.chars()
            .filter(|c| c.is_digit(10))
            .fold(String::new(), |mut acc, c| {
                if acc.len() <= 1 {
                    acc.push(c);
                    return acc;
                }
                let mut new_acc = String::new();
                new_acc.push(acc.chars().rev().last().unwrap());
                new_acc.push(c);
                return new_acc;
            })
            .parse::<i32>()
            .map(|num| {
                if num < 10 {
                    return num * 10 + num;
                }
                return num;
            })
            .unwrap()
    }
}

fn first_solution(input: &str) -> i32 {
    return input
        .split('\n')
        .filter(|line| !line.is_empty())
        .fold(0, |sum, line| sum + line.get_calibration_value());
}

fn second_solution(input: &str) -> i32 {
    let x = input
        .split('\n')
        .filter(|line| !line.is_empty())
        .map(|line| line.word_to_int());
    println!("x: {:#?}", &x.clone().collect::<String>());
    return x.fold(0, |sum, line| sum + line.get_calibration_value());
}

fn main() -> Result<(), Box<dyn Error>> {
    // let part1_sample_output = first_solution(include_str!("../part1-sample.txt"));
    // let part1_real_output = first_solution(include_str!("../part1-real.txt"));

    let part2_sample_output = second_solution(include_str!("../part2-sample.txt"));
    // let part2_real_output = second_solution(include_str!("../part2-real.txt"));

    // println!("part1-sample: {:#?}", part1_sample_output);
    // println!("part1-real: {:#?}", part1_real_output);

    println!("part2-sample: {:#?}", part2_sample_output);
    // println!("part2-real: {:#?}", part2_real_output);
    return Ok(());
}
