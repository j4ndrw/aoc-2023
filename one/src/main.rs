use std::error::Error;

trait Calibratable {
    fn convert_words_to_digits(&self) -> String;
    fn get_last_digit_from_worded_number(&self) -> Option<char>;
    fn get_calibration_value(&self) -> i32;
}

impl Calibratable for str {
    fn get_last_digit_from_worded_number(&self) -> Option<char> {
        if self.len() < 3 {
            return None;
        }
        return match self {
            "one" => Some('1'),
            "two" => Some('2'),
            "three" => Some('3'),
            "four" => Some('4'),
            "five" => Some('5'),
            "six" => Some('6'),
            "seven" => Some('7'),
            "eight" => Some('8'),
            "nine" => Some('9'),
            s => s
                .chars()
                .skip(1)
                .collect::<String>()
                .get_last_digit_from_worded_number(),
        };
    }
    fn convert_words_to_digits(&self) -> String {
        let mut buf = String::new();
        let mut new_str = String::new();

        for c in self.chars() {
            buf.push(c);

            let buf_snapshot = buf.clone();
            let digit = buf_snapshot.get_last_digit_from_worded_number();

            let aggregate = if let Some(digit) = digit {
                Some(digit)
            } else if c.is_digit(10) {
                Some(c)
            } else {
                None
            };

            if let Some(aggregate) = aggregate {
                buf.clear();
                buf.push(buf_snapshot.chars().last().unwrap());
                new_str.push(aggregate);
            }
        }
        return new_str;
    }

    fn get_calibration_value(&self) -> i32 {
        self.chars()
            .fold(String::new(), |mut acc, c| {
                if !c.is_digit(10) {
                    return acc;
                }
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
    return input
        .split('\n')
        .filter(|line| !line.is_empty())
        .map(|line| line.convert_words_to_digits())
        .fold(0, |sum, line| sum + line.get_calibration_value());
}

fn main() -> Result<(), Box<dyn Error>> {
    let part1_sample_output = first_solution(include_str!("../part1-sample.txt"));
    let part1_real_output = first_solution(include_str!("../part1-real.txt"));

    let part2_sample_output = second_solution(include_str!("../part2-sample.txt"));
    let part2_real_output = second_solution(include_str!("../part2-real.txt"));

    println!("part1-sample: {:#?}", part1_sample_output);
    println!("part1-real: {:#?}", part1_real_output);

    println!("part2-sample: {:#?}", part2_sample_output);
    println!("part2-real: {:#?}", part2_real_output);

    return Ok(());
}
