#[derive(Clone, Copy)]
enum Play {
    Rock,
    Paper,
    Scissors,
}

#[derive(Clone, Copy)]
enum Outcome {
    Win,
    Tie,
    Lose,
}

impl Outcome {
    fn score(self) -> u32 {
        match self {
            Outcome::Win => 6,
            Outcome::Tie => 3,
            Outcome::Lose => 0,
        }
    }

    fn from_letter(letter: char) -> Self {
        match letter {
            'X' => Outcome::Lose,
            'Y' => Outcome::Tie,
            'Z' => Outcome::Win,
            _ => panic!(),
        }
    }
}

impl Play {
    fn fight(self, other: Play) -> Outcome {
        match (self, other) {
            (Play::Rock, Play::Scissors) => Outcome::Win,
            (Play::Rock, Play::Paper) => Outcome::Lose,
            (Play::Paper, Play::Rock) => Outcome::Win,
            (Play::Paper, Play::Scissors) => Outcome::Lose,
            (Play::Scissors, Play::Paper) => Outcome::Win,
            (Play::Scissors, Play::Rock) => Outcome::Lose,
            _ => Outcome::Tie,
        }
    }

    fn find_play(self, outcome: Outcome) -> Self {
        match (self, outcome) {
            (Play::Rock, Outcome::Win) => Play::Paper,
            (Play::Rock, Outcome::Tie) => Play::Rock,
            (Play::Rock, Outcome::Lose) => Play::Scissors,
            (Play::Paper, Outcome::Win) => Play::Scissors,
            (Play::Paper, Outcome::Tie) => Play::Paper,
            (Play::Paper, Outcome::Lose) => Play::Rock,
            (Play::Scissors, Outcome::Win) => Play::Rock,
            (Play::Scissors, Outcome::Tie) => Play::Scissors,
            (Play::Scissors, Outcome::Lose) => Play::Paper,
        }
    }

    fn score(self) -> u32 {
        match self {
            Play::Rock => 1,
            Play::Paper => 2,
            Play::Scissors => 3,
        }
    }

    fn from_letter(letter: char) -> Self {
        match letter {
            'A' => Play::Rock,
            'B' => Play::Paper,
            'C' => Play::Scissors,
            'X' => Play::Rock,
            'Y' => Play::Paper,
            'Z' => Play::Scissors,
            _ => panic!(),
        }
    }
}

pub fn run(input: &str) -> (u32, u32) {
    let mut score1 = 0;
    let mut score2 = 0;
    for line in input.lines() {
        let them = Play::from_letter(line.chars().nth(0).unwrap());
        let me = Play::from_letter(line.chars().nth(2).unwrap());
        score1 += me.score();
        score1 += me.fight(them).score();

        let desired_outcome = Outcome::from_letter(line.chars().nth(2).unwrap());
        let actual_me = them.find_play(desired_outcome);
        score2 += actual_me.score();
        score2 += desired_outcome.score();
    }
    (score1, score2)
}
