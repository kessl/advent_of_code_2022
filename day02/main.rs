enum Move {
    Rock,
    Paper,
    Scissors,
}
use Move::*;

struct Round {
    opponent: Move,
    player: Move,
}

impl Round {
    // score: (0 lost, 3 draw, 6 win) + (1 rock, 2 paper, 3 scissors)
    fn fight(&self) -> u32 {
        match (&self.opponent, &self.player) {
            (Rock, Rock) => 3 + 1,
            (Rock, Paper) => 6 + 2,
            (Rock, Scissors) => 0 + 3,
            (Paper, Rock) => 0 + 1,
            (Paper, Paper) => 3 + 2,
            (Paper, Scissors) => 6 + 3,
            (Scissors, Rock) => 6 + 1,
            (Scissors, Paper) => 0 + 2,
            (Scissors, Scissors) => 3 + 3,
        }
    }
}

fn from_a(value: &str) -> Round {
    let (opponent_str, player_str) = value.split_once(' ').expect("2 letters separated by whitespace");
    let opponent = match opponent_str {
        "A" => Rock,
        "B" => Paper,
        "C" => Scissors,
        _ => panic!("Unknown move"),
    };
    let player = match player_str {
        "X" => Rock,
        "Y" => Paper,
        "Z" => Scissors,
        _ => panic!("Unknown move"),
    };
    Round { opponent, player }
}

fn from_b(value: &str) -> Round {
    let (opponent_str, outcome_str) = value.split_once(' ').expect("2 letters separated by whitespace");
    let opponent = match opponent_str {
        "A" => Rock,
        "B" => Paper,
        "C" => Scissors,
        _ => panic!("Unknown move"),
    };

    // X = lose, Y = draw, Z = win
    let player = match (&opponent, outcome_str) {
        (Rock, "X") => Scissors,
        (Rock, "Y") => Rock,
        (Rock, "Z") => Paper,
        (Paper, "X") => Rock,
        (Paper, "Y") => Paper,
        (Paper, "Z") => Scissors,
        (Scissors, "X") => Paper,
        (Scissors, "Y") => Scissors,
        (Scissors, "Z") => Rock,
        _ => panic!("Unknown move"),
    };
    Round { opponent, player }
}

pub fn main() {
    let input = include_str!("./input.txt").lines();
    
    let score_a: u32 = input
        .clone()
        .map(from_a)
        .map(|round| round.fight())
        .sum();
    println!("dayO2a: {score_a}");

    let score_b: u32 = input
        .map(from_b)
        .map(|round| round.fight())
        .sum();
    println!("dayO2b: {score_b}");
}

