use crate::days::day2::Play::{Paper, Rock, Scissors};
use crate::days::day2::RoundResult::{Draw, Lose, Win};

#[derive(PartialEq, Eq, Copy, Clone)]
enum Play {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

#[derive(PartialEq, Eq)]
enum RoundResult {
    Lose,
    Draw,
    Win,
}

struct PlayRound {
    opponent: Play,
    response: Option<Play>,
    round_result: Option<RoundResult>,
    resolved: bool,
}

impl PlayRound {
    fn resolve(&mut self) {
        match &self.round_result {
            None => {
                if (self.opponent == Paper && self.response.unwrap() == Rock)
                    || (self.opponent == Scissors && self.response.unwrap() == Paper)
                    || (self.opponent == Rock && self.response.unwrap() == Scissors)
                {
                    self.round_result = Some(Lose);
                } else if self.opponent == self.response.unwrap() {
                    self.round_result = Some(Draw);
                } else {
                    self.round_result = Some(Win);
                }
            }
            Some(res) => {
                if *res == Lose && self.opponent == Rock {
                    self.response = Some(Scissors);
                } else if *res == Lose && self.opponent == Paper {
                    self.response = Some(Rock);
                } else if *res == Lose && self.opponent == Scissors {
                    self.response = Some(Paper);
                } else if *res == Draw && self.opponent == Rock {
                    self.response = Some(Rock);
                } else if *res == Draw && self.opponent == Paper {
                    self.response = Some(Paper);
                } else if *res == Draw && self.opponent == Scissors {
                    self.response = Some(Scissors);
                } else if *res == Win && self.opponent == Rock {
                    self.response = Some(Paper);
                } else if *res == Win && self.opponent == Paper {
                    self.response = Some(Scissors);
                } else if *res == Win && self.opponent == Scissors {
                    self.response = Some(Rock);
                }
            }
        }
        self.resolved = true
    }

    fn round_score(&mut self) -> i32 {
        if !self.resolved {
            self.resolve();
        }
        match &self.round_result {
            None => {
                panic!("We can't have a round without a result")
            }
            Some(round_result) => {
                if *round_result == Lose {
                    self.response.unwrap() as i32
                } else if *round_result == Draw {
                    3 + self.response.unwrap() as i32
                } else {
                    6 + self.response.unwrap() as i32
                }
            }
        }
    }
}

fn part1(input_file: &str) {
    let rounds: Vec<PlayRound> = input_file
        .lines()
        .map(|val| {
            let line_split: Vec<&str> = val.split(' ').collect();
            let opponent: Play = match line_split[0] {
                "A" => Rock,
                "B" => Paper,
                "C" => Scissors,
                _ => Scissors,
            };
            let response: Play = match line_split[1] {
                "X" => Rock,
                "Y" => Paper,
                "Z" => Scissors,
                _ => Scissors,
            };
            PlayRound {
                opponent,
                response: Some(response),
                round_result: None,
                resolved: false,
            }
        })
        .collect();
    let mut total_score = 0;
    for mut round in rounds {
        total_score += round.round_score()
    }
    println!("Part 1: {}", total_score);
}

fn part2(input_file: &str) {
    let rounds: Vec<PlayRound> = input_file
        .lines()
        .map(|val| {
            let line_split: Vec<&str> = val.split(' ').collect();
            let opponent: Play = match line_split[0] {
                "A" => Rock,
                "B" => Paper,
                "C" => Scissors,
                _ => Scissors,
            };
            let round_result: RoundResult = match line_split[1] {
                "X" => Lose,
                "Y" => Draw,
                "Z" => Win,
                _ => Draw,
            };
            PlayRound {
                opponent,
                response: None,
                round_result: Some(round_result),
                resolved: false,
            }
        })
        .collect();
    let mut total_score = 0;
    for mut round in rounds {
        total_score += round.round_score()
    }
    println!("Part 2: {}", total_score);
}

pub fn run(payload: &str) {
    part1(payload);
    part2(payload);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::days::day2::Play::{Paper, Rock, Scissors};
    use rstest::rstest;

    #[rstest]
    // draws
    #[case(Rock, Some(Rock), None, 4)]
    #[case(Paper, Some(Paper), None, 5)]
    #[case(Scissors, Some(Scissors), None, 6)]
    // wins
    #[case(Rock, Some(Paper), None, 8)]
    #[case(Paper, Some(Scissors), None, 9)]
    #[case(Scissors, Some(Rock), None, 7)]
    // loses
    #[case(Paper, Some(Rock), None, 1)]
    #[case(Scissors, Some(Paper), None, 2)]
    #[case(Rock, Some(Scissors), None, 3)]
    fn play_round_score_tests(
        #[case] opponent: Play,
        #[case] response: Option<Play>,
        #[case] round_result: Option<RoundResult>,
        #[case] expected: i32,
    ) {
        assert_eq!(
            expected,
            PlayRound {
                opponent,
                response,
                round_result,
                resolved: false,
            }
            .round_score()
        )
    }
}
