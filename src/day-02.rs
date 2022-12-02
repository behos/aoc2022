use std::fs::read_to_string;

enum Outcome {
    Win,
    Lose,
    Draw,
}

impl Outcome {
    fn score(&self) -> usize {
        match self {
            Self::Win => 6,
            Self::Draw => 3,
            Self::Lose => 0,
        }
    }
}

#[derive(PartialEq, Eq, Clone, Copy)]
enum Play {
    Rock,
    Paper,
    Scissors,
}

impl Play {
    fn against(&self, opponent: &Play) -> Outcome {
        match opponent {
            b if b == &self.beats() => Outcome::Win,
            b if self == b => Outcome::Draw,
            _ => Outcome::Lose,
        }
    }

    fn score(&self) -> usize {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }
    }

    fn beats(&self) -> Self {
        match self {
            Self::Paper => Self::Rock,
            Self::Rock => Self::Scissors,
            Self::Scissors => Self::Paper,
        }
    }

    fn loses_to(&self) -> Self {
        match self {
            Self::Rock => Self::Paper,
            Self::Scissors => Self::Rock,
            Self::Paper => Self::Scissors,
        }
    }

    fn for_outcome(&self, outcome: Outcome) -> Self {
        match outcome {
            Outcome::Draw => *self,
            Outcome::Win => self.beats(),
            Outcome::Lose => self.loses_to(),
        }
    }
}

struct Game {
    you: Play,
    opponent: Play,
}

impl Game {
    fn score(&self) -> usize {
        self.you.against(&self.opponent).score() + self.you.score()
    }

    fn from_strategy_1(line: &str) -> Self {
        let mut parts = line.split(' ');
        Game {
            opponent: match parts.next() {
                Some("A") => Play::Rock,
                Some("B") => Play::Paper,
                Some("C") => Play::Scissors,
                x => panic!("unexpected input {x:?}"),
            },
            you: match parts.next() {
                Some("X") => Play::Rock,
                Some("Y") => Play::Paper,
                Some("Z") => Play::Scissors,
                x => panic!("unexpected input {x:?}"),
            },
        }
    }

    fn from_strategy_2(line: &str) -> Self {
        let mut parts = line.split(' ');
        let opponent = match parts.next() {
            Some("A") => Play::Rock,
            Some("B") => Play::Paper,
            Some("C") => Play::Scissors,
            x => panic!("unexpected input {x:?}"),
        };
        let you = opponent.for_outcome(match parts.next() {
            Some("X") => Outcome::Win,
            Some("Y") => Outcome::Draw,
            Some("Z") => Outcome::Lose,
            x => panic!("unexpected input {x:?}"),
        });

        Game {
            opponent,
            you,
        }
    }
}

fn main() {
    let input = read_to_string("inputs/day-02.txt").expect("missing input file");
    println!("Part 1: {}", total_score(&input, Game::from_strategy_1));
    println!("Part 2: {}", total_score(&input, Game::from_strategy_2));
}

fn total_score(input: &str, strategy: fn(&str) -> Game) -> usize {
    input
        .split('\n')
        .filter_map(|s| {
            if s.is_empty() {
                None
            } else {
                Some(strategy(s).score())
            }
        })
        .sum()
}
