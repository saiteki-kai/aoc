// https://adventofcode.com/2022/day/2
//
// Day 2: Rock Paper Scissors - Part 1

use super::shared::*;

fn get_shape(code: &str) -> Shape {
    match code.trim() {
        "A" => Shape::Rock,
        "B" => Shape::Paper,
        "C" => Shape::Scissor,
        "X" => Shape::Rock,
        "Y" => Shape::Paper,
        "Z" => Shape::Scissor,
        _ => panic!("Unknown Code"),
    }
}

fn map_shapes(input: &str) -> (Shape, Shape) {
    let (s1, s2) = input.split_at(1);

    let shape1 = get_shape(s1);
    let shape2 = get_shape(s2);

    (shape1, shape2)
}

fn get_rounds() -> Vec<(Shape, Shape)> {
    let rounds = include_str!("../../../inputs/day02")
        .trim()
        .split('\n')
        .map(map_shapes)
        .collect();

    rounds
}

fn compute_total_score(rounds: Vec<(Shape, Shape)>) -> u32 {
    let mut total_score: u32 = 0;

    for (oppenent_shape, player_shape) in rounds {
        let outcome = get_outcome(player_shape, oppenent_shape);
        let round_score = compute_round_score(player_shape, outcome);

        total_score += round_score;
    }

    total_score
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
        let rounds: Vec<(Shape, Shape)> = ["A Y", "B X", "C Z"].map(map_shapes).to_vec();
        let total_score = compute_total_score(rounds);

        assert_eq!(total_score, 15);
    }
}
