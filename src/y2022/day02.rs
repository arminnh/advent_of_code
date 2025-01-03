#[derive(Debug)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

enum Outcome {
    Win,
    Lose,
    Draw,
}

fn parse_move(c: &char) -> Option<Move> {
    match c {
        'A' | 'X' => Some(Move::Rock),
        'B' | 'Y' => Some(Move::Paper),
        'C' | 'Z' => Some(Move::Scissors),
        _ => None,
    }
}

fn parse_outcome(c: &char) -> Option<Outcome> {
    match c {
        'X' => Some(Outcome::Lose),
        'Y' => Some(Outcome::Draw),
        'Z' => Some(Outcome::Win),
        _ => None,
    }
}

// The score for a single round is the score for the shape you selected
// (1 for Rock, 2 for Paper, and 3 for Scissors) plus the score for the outcome of
// the round (0 if you lost, 3 if the round was a draw, and 6 if you won).
fn calculate_score(opponent_move: &Move, my_move: &Move) -> u32 {
    match opponent_move {
        Move::Rock => match my_move {
            Move::Rock => 3 + 1,
            Move::Paper => 6 + 2,
            Move::Scissors => 0 + 3,
        },
        Move::Paper => match my_move {
            Move::Rock => 0 + 1,
            Move::Paper => 3 + 2,
            Move::Scissors => 6 + 3,
        },
        Move::Scissors => match my_move {
            Move::Rock => 6 + 1,
            Move::Paper => 0 + 2,
            Move::Scissors => 3 + 3,
        },
    }
}

// The total score is still calculated in the same way, but now you need to figure out
// what shape to choose so the round ends as indicated.
fn select_move_for_target_outcome(opponent_move: &Move, target_outcome: &Outcome) -> Move {
    match opponent_move {
        Move::Rock => match target_outcome {
            Outcome::Win => Move::Paper,
            Outcome::Lose => Move::Scissors,
            Outcome::Draw => Move::Rock,
        },
        Move::Paper => match target_outcome {
            Outcome::Win => Move::Scissors,
            Outcome::Lose => Move::Rock,
            Outcome::Draw => Move::Paper,
        },
        Move::Scissors => match target_outcome {
            Outcome::Win => Move::Rock,
            Outcome::Lose => Move::Paper,
            Outcome::Draw => Move::Scissors,
        },
    }
}

pub fn part_1(input: &str) -> u32 {
    0
}

// What would your total score be if everything goes exactly according to your strategy guide?
pub fn part_2(input: &str) -> u32 {
    input.lines().fold(0, |acc, line| {
        let opponent_move: Move = parse_move(&line.chars().nth(0).unwrap()).unwrap();
        // let my_move: Move = parse_move(line.chars().nth(2).unwrap()).unwrap();
        let target_outcome: Outcome = parse_outcome(&line.chars().nth(2).unwrap()).unwrap();
        let my_move: Move = select_move_for_target_outcome(&opponent_move, &target_outcome);
        acc + calculate_score(&opponent_move, &my_move)
    })
}
