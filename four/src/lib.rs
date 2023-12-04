use std::{error::Error, rc::Rc, str::FromStr};

type Card = i32;

const MULTIPLIER: i32 = 2;

#[derive(Debug)]
struct ScratchCard {
    targets: Vec<Card>,
    queries: Vec<Card>,
}

#[derive(Debug)]
struct WinningNumbers(Vec<Card>);

impl ScratchCard {
    fn winning_numbers(&self) -> WinningNumbers {
        // Linear search is slow, but should be fine for AoC
        return WinningNumbers(self.queries.iter().fold(Vec::default(), |mut acc, query| {
            if !self.targets.contains(query) {
                return acc;
            }
            acc.push(*query);
            return acc;
        }));
    }

    fn calculate_copies(
        scratch_cards: Vec<Rc<ScratchCard>>,
        mut aggregate: Vec<Rc<ScratchCard>>,
    ) -> i32 {
        for (index, scratch_card) in scratch_cards.iter().enumerate() {
            let WinningNumbers(winning_numbers) = scratch_card.winning_numbers();
            let many = winning_numbers.len();

            if many == 0 {
                continue;
            }

            for id in index..many {
                if let Some(copy) = scratch_cards.get(id) {
                    aggregate.push(copy.clone());
                }
            }
        }
        // let _ = ScratchCard::calculate_copies(aggregate.clone(), vec![]);
        return aggregate.len() as i32;
    }
}

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

impl FromStr for ScratchCard {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (_, rest) = s.split_once(": ").unwrap();
        let (queries, targets) = rest.split_once(" | ").unwrap();
        let queries = queries
            .split(' ')
            .map(|query| query.trim())
            .filter(|query| !query.is_empty())
            .map(|query| query.parse::<Card>().unwrap())
            .collect();
        let targets = targets
            .split(' ')
            .map(|target| target.trim())
            .filter(|target| !target.is_empty())
            .map(|target| target.parse::<Card>().unwrap())
            .collect();
        return Ok(ScratchCard { queries, targets });
    }
}

pub fn first_solution(input: &str) -> i32 {
    return input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            line.parse::<ScratchCard>()
                .unwrap()
                .winning_numbers()
                .points()
        })
        .sum();
}

pub fn second_solution(input: &str) -> i32 {
    return ScratchCard::calculate_copies(
        input
            .lines()
            .filter(|line| !line.is_empty())
            .map(|line| line.parse::<ScratchCard>().unwrap())
            .map(|scratch_card| Rc::new(scratch_card))
            .collect(),
        vec![],
    );
}
