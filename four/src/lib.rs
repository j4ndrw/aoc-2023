use std::{collections::HashSet, error::Error, rc::Rc, str::FromStr};

const MULTIPLIER: i32 = 2;

type Card = i32;

#[derive(Debug)]
struct ScratchCard {
    targets: HashSet<Card>,
    queries: HashSet<Card>,
}
impl ScratchCard {
    fn winning_numbers(&self) -> WinningNumbers {
        return WinningNumbers(self.targets.intersection(&self.queries).cloned().collect());
    }

    fn calculate_copies(scratch_cards: Vec<ScratchCard>) -> i32 {
        let mut copies = vec![1; scratch_cards.len()];
        for (start, scratch_card) in scratch_cards.iter().enumerate() {
            let next = start + 1;

            let WinningNumbers(winning_numbers) = scratch_card.winning_numbers();
            for (win_index, _) in winning_numbers.iter().enumerate() {
                copies[next + win_index] += copies[start];
            }
        }
        return copies.into_iter().reduce(|acc, copy| acc + copy).unwrap() as i32;
    }

    // My brain hurts, so no recursion. Going full imperative on this one.
    #[allow(dead_code)]
    fn calculate_copies_rec(scratch_cards: Vec<Rc<ScratchCard>>, mut sum: i32) -> i32 {
        if scratch_cards.len() == 0 {
            return sum;
        }

        for (start, scratch_card) in scratch_cards.iter().enumerate() {
            let WinningNumbers(winning_numbers) = scratch_card.winning_numbers();
            if winning_numbers.len() == 0 {
                continue;
            }

            let start = start + 1;
            let end = start + winning_numbers.len();

            if start > end || end > scratch_cards.len() {
                continue;
            }

            let copies = scratch_cards[start..end].to_vec();
            sum += copies.len() as i32
                + ScratchCard::calculate_copies_rec(copies.clone(), copies.len() as i32);
        }
        return sum;
    }
}

impl FromStr for ScratchCard {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.split_once(": ").unwrap().1;
        let (queries, targets) = s.split_once(" | ").unwrap();
        let queries = queries
            .split(' ')
            .map(|query| query.trim())
            .filter_map(|query| match query.parse::<Card>() {
                Ok(query) => Some(query),
                Err(_) => None,
            })
            .collect();
        let targets = targets
            .split(' ')
            .map(|target| target.trim())
            .filter_map(|target| match target.parse::<Card>() {
                Ok(target) => Some(target),
                Err(_) => None,
            })
            .collect();
        return Ok(ScratchCard { queries, targets });
    }
}

#[derive(Debug)]
struct WinningNumbers(HashSet<Card>);
impl WinningNumbers {
    fn points(&self) -> i32 {
        let WinningNumbers(winning_numbers) = self;
        return winning_numbers
            .iter()
            .enumerate()
            .fold(0, |mut acc, (index, _)| {
                if index == 0 {
                    acc += 1;
                    return acc;
                }
                acc *= MULTIPLIER;
                return acc;
            });
    }
}

pub fn first_solution(input: &str) -> i32 {
    return input
        .lines()
        .filter_map(|line| match line.parse::<ScratchCard>() {
            Ok(scratch_card) => Some(scratch_card.winning_numbers().points()),
            Err(_) => None,
        })
        .sum();
}

pub fn second_solution(input: &str) -> i32 {
    return ScratchCard::calculate_copies(
        input
            .lines()
            .filter_map(|line| match line.parse::<ScratchCard>() {
                Ok(scratch_card) => Some(scratch_card),
                Err(_) => None,
            })
            .collect(),
    );
}
