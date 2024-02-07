// https://adventofcode.com/2022/2
//
// Day 2: Rock Paper Scissors - Part 2

use super::shared::*;

fn get_opponent_shape(code: &str) -> Shape {
    match code.trim() {
        "A" => Shape::Rock,
        "B" => Shape::Paper,
        "C" => Shape::Scissor,
        _ => panic!("Unknown Code"),
    }
}

fn get_player_shape(oppenent_shape: Shape, outcome: Outcome) -> Shape {
    let shape = match (outcome, oppenent_shape) {
        (Outcome::Win, Shape::Rock) => Shape::Paper,
        (Outcome::Win, Shape::Paper) => Shape::Scissor,
        (Outcome::Win, Shape::Scissor) => Shape::Rock,
        (Outcome::Lost, Shape::Rock) => Shape::Scissor,
        (Outcome::Lost, Shape::Paper) => Shape::Rock,
        (Outcome::Lost, Shape::Scissor) => Shape::Paper,
        _ => oppenent_shape, // draw
    };

    return shape;
}

fn get_outcome(code: &str) -> Outcome {
    match code.trim() {
        "X" => Outcome::Win,
        "Y" => Outcome::Draw,
        "Z" => Outcome::Lost,
        _ => panic!("Unknown Code"),
    }
}

fn map_shapes(input: &str) -> (Shape, Shape, Outcome) {
    let (s1, s2) = input.split_at(1);

    let outcome = get_outcome(s2);

    let shape1 = get_opponent_shape(s1);
    let shape2 = get_player_shape(shape1, outcome);

    return (shape1, shape2, outcome);
}

fn get_rounds() -> Vec<(Shape, Shape, Outcome)> {
    let rounds = include_str!("../../../inputs/day02")
        .trim()
        .split("\n")
        .map(map_shapes)
        .collect();

    return rounds;
}

fn compute_total_score(rounds: Vec<(Shape, Shape, Outcome)>) -> u32 {
    let mut total_score: u32 = 0;

    for (_, player_shape, outcome) in rounds {
        let round_score = compute_round_score(player_shape, outcome);

        total_score += round_score;
    }

    return total_score;
}

pub fn solve() {
    let rounds = get_rounds();
    let total_score: u32 = compute_total_score(rounds);
    println!("{total_score}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_total_score() {
        let rounds: Vec<(Shape, Shape, Outcome)> = ["A Y", "B X", "C Z"].map(map_shapes).to_vec();

        for (s1, s2, o) in rounds {
            println!("{:?} {:?} {:?}", s1, s2, o);
        }

        // let total_score = compute_total_score(rounds);

        // assert_eq!(total_score, 12);
    }
}
