use std::error::Error;

trait Trebuchet {
    fn map_words_to_digits(&self) -> String;
    fn word_to_digit(&self) -> Option<char>;
    fn calibration(&self) -> Result<i32, Box<dyn Error>>;
}

impl Trebuchet for str {
    fn word_to_digit(&self) -> Option<char> {
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
            s => s.chars().skip(1).collect::<String>().word_to_digit(),
        };
    }
    fn map_words_to_digits(&self) -> String {
        let (_, new_str) = self.chars().fold(
            (String::default(), String::default()),
            |(mut buf, mut new_str), c| {
                buf.push(c);
                buf.word_to_digit()
                    .or(if c.is_digit(10) { Some(c) } else { None })
                    .map(|digit| {
                        let last_char = buf.chars().last().unwrap();

                        buf = last_char.to_string();
                        new_str.push(digit)
                    });
                return (buf, new_str);
            },
        );

        return new_str;
    }

    fn calibration(&self) -> Result<i32, Box<dyn Error>> {
        let number = self
            .chars()
            .filter(|c| c.is_digit(10))
            .fold(String::default(), |mut acc, c| {
                if acc.len() <= 1 {
                    acc.push(c);
                    return acc;
                }
                let last_digit = acc.chars().rev().last().unwrap();
                return format!("{last_digit}{c}");
            })
            .parse::<i32>()?;

        return Ok(match number < 10 {
            true => number * 10 + number,
            false => number,
        });
    }
}

pub fn first_solution(input: &str) -> i32 {
    return input
        .lines()
        .filter_map(|line| match line.calibration() {
            Ok(calibration) => Some(calibration),
            Err(_) => None,
        })
        .sum();
}

pub fn second_solution(input: &str) -> i32 {
    return input
        .lines()
        .filter_map(|line| match line.map_words_to_digits().calibration() {
            Ok(calibration) => Some(calibration),
            Err(_) => None,
        })
        .sum();
}
