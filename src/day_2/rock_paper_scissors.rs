use crate::file_reader;

#[derive(PartialEq, Eq)]
enum Play {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

#[derive(PartialEq, Eq)]
enum Result {
    Lose,
    Draw,
    Win,
}

struct UnexplainedStrategy {
    opponent: Play,
    me: Play,
}

struct ExplainedStrategy {
    opponent: Play,
    me: Result,
}

fn print_score_for_unexplained_starts(strats: Vec<UnexplainedStrategy>) {
    let mut score = 0;

    for strat in strats {
        // Win.
        if match strat.me {
            Play::Rock => strat.opponent == Play::Scissors,
            Play::Paper => strat.opponent == Play::Rock,
            Play::Scissors => strat.opponent == Play::Paper,
        } {
            score += 6 + strat.me as i32;
            continue;
        }

        // Lose.
        if match strat.me {
            Play::Rock => strat.opponent == Play::Paper,
            Play::Paper => strat.opponent == Play::Scissors,
            Play::Scissors => strat.opponent == Play::Rock,
        } {
            score += 0 + strat.me as i32;
            continue;
        }

        // Draw.
        if strat.me == strat.opponent {
            score += 3 + strat.me as i32;
            continue;
        }
    }

    println!("Day 02: Part 1: {}", score);
}

fn print_score_for_explained_starts(strats: Vec<ExplainedStrategy>) {
    let mut score = 0;

    for strat in strats {
        // Win.
        if strat.me == Result::Win {
            let response_play: Play = match strat.opponent {
                Play::Rock => Play::Paper,
                Play::Paper => Play::Scissors,
                Play::Scissors => Play::Rock,
            };

            score += 6 + response_play as i32;
            continue;
        }

        // Lose.
        if strat.me == Result::Lose {
            let response_play: Play = match strat.opponent {
                Play::Rock => Play::Scissors,
                Play::Paper => Play::Rock,
                Play::Scissors => Play::Paper,
            };

            score += 0 + response_play as i32;
            continue;
        }

        // Draw.
        if strat.me == Result::Draw {
            score += 3 + strat.opponent as i32;
            continue;
        }
    }

    println!("Day 02: Part 2: {}", score);
}

fn get_unexplained_start_from_str(s: &str) -> UnexplainedStrategy {
    let strat_str: Vec<&str> = s.split(' ').collect();

    let opponent: Play = match strat_str[0] {
        "A" => Play::Rock,
        "B" => Play::Paper,
        "C" => Play::Scissors,
        _ => panic!("Invalid input"),
    };

    let me: Play = match strat_str[1] {
        "X" => Play::Rock,
        "Y" => Play::Paper,
        "Z" => Play::Scissors,
        _ => panic!("Invalid input"),
    };

    UnexplainedStrategy { opponent, me }
}

fn get_explained_start_from_str(s: &str) -> ExplainedStrategy {
    let strat_str: Vec<&str> = s.split(' ').collect();

    let opponent: Play = match strat_str[0] {
        "A" => Play::Rock,
        "B" => Play::Paper,
        "C" => Play::Scissors,
        _ => panic!("Invalid input"),
    };

    let me: Result = match strat_str[1] {
        "X" => Result::Lose,
        "Y" => Result::Draw,
        "Z" => Result::Win,
        _ => panic!("Invalid input"),
    };

    ExplainedStrategy { opponent, me }
}

pub fn run() {
    let input = file_reader::read_file_in_cwd("src/day_2/input.txt");

    let lines: Vec<&str> = input.split('\n').collect();

    let unexplained_strats: Vec<UnexplainedStrategy> = lines
        .iter()
        .map(|&val| get_unexplained_start_from_str(val))
        .collect();

    let explained_strats: Vec<ExplainedStrategy> = lines
        .iter()
        .map(|&val| get_explained_start_from_str(val))
        .collect();

    print_score_for_unexplained_starts(unexplained_strats);

    print_score_for_explained_starts(explained_strats);
}
