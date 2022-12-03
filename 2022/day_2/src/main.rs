use std::fs;

//const FILE: &str = "test.txt";
const FILE: &str = "final.txt";

#[derive(Debug)]
enum Throw {
    Rock,
    Paper,
    Scissors,
}

enum GameResult {
    Win,
    Lose,
    Draw,
}

fn play(theirs: &Throw, ours: &Throw) -> GameResult {
    match theirs {
        Throw::Rock => match ours {
            Throw::Rock => GameResult::Draw,
            Throw::Paper => GameResult::Win,
            Throw::Scissors => GameResult::Lose,
        },
        Throw::Paper => match ours {
            Throw::Rock => GameResult::Lose,
            Throw::Paper => GameResult::Draw,
            Throw::Scissors => GameResult::Win,
        },
        Throw::Scissors => match ours {
            Throw::Rock => GameResult::Win,
            Throw::Paper => GameResult::Lose,
            Throw::Scissors => GameResult::Draw,
        },
    }
}

fn score(theirs: &Throw, ours: &Throw) -> u32 {
    let result = play(theirs, ours); 

    let mut score = match ours {
        Throw::Rock => 1,
        Throw::Paper => 2,
        Throw::Scissors => 3,
    };

    score += match result {
        GameResult::Win => 6,
        GameResult::Draw => 3,
        GameResult::Lose => 0,
    };

    score
}

fn parse_throw(throw_repr: &str) -> Option<Throw> {
    match throw_repr {
        "A" | "X" => Some(Throw::Rock),
        "B" | "Y" => Some(Throw::Paper),
        "C" | "Z" => Some(Throw::Scissors),
        _ => None,
    }
}

fn parse_outcome(outcome_repr: &str) -> Option<GameResult> {
    match outcome_repr {
        "X" => Some(GameResult::Lose),
        "Y" => Some(GameResult::Draw),
        "Z" => Some(GameResult::Win),
        _ => None,
    }
}

fn infer_throw(theirs: &Throw, outcome: &GameResult) -> Throw {
    match theirs {
        Throw::Rock => match outcome {
            GameResult::Lose => Throw::Scissors,
            GameResult::Draw => Throw::Rock,
            GameResult::Win => Throw::Paper,
        },
        Throw::Paper => match outcome {
            GameResult::Lose => Throw::Rock,
            GameResult::Draw => Throw::Paper,
            GameResult::Win => Throw::Scissors,
        },
        Throw::Scissors => match outcome {
            GameResult::Lose => Throw::Paper,
            GameResult::Draw => Throw::Scissors,
            GameResult::Win => Throw::Rock,
        },
    }
}

fn main() {
    let contents = fs::read_to_string(FILE).unwrap();
    let lines = contents.lines();

    let mut total_score: u32 = 0;
    for line in lines {
        let mut split = line.split_whitespace();
        let theirs = parse_throw(split.next().unwrap()).unwrap();
        let outcome = parse_outcome(split.next().unwrap()).unwrap();
        let ours = infer_throw(&theirs, &outcome);

        let match_score = score(&theirs, &ours);
        println!("Game:  {:?}  {:?}, Score: {}", theirs, ours, match_score);
        total_score += match_score;
    }
    println!("final score: {}", total_score);
}
