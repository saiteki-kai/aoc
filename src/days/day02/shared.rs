#[derive(PartialEq, Debug, Clone, Copy)]
pub enum Shape {
    Rock,
    Paper,
    Scissor,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Outcome {
    Win,
    Draw,
    Lost,
}

pub fn get_outcome(player: Shape, oppenent: Shape) -> Outcome {
    if player == oppenent {
        return Outcome::Draw;
    }

    match (player, oppenent) {
        (Shape::Paper, Shape::Rock) => Outcome::Win,
        (Shape::Rock, Shape::Scissor) => Outcome::Win,
        (Shape::Scissor, Shape::Paper) => Outcome::Win,
        _ => Outcome::Lost,
    }
}

pub fn compute_round_score(s: Shape, o: Outcome) -> u32 {
    let shape_score: u32 = match s {
        Shape::Rock => 1,
        Shape::Paper => 2,
        Shape::Scissor => 3,
    };

    let outcome_score: u32 = match o {
        Outcome::Lost => 0,
        Outcome::Draw => 3,
        Outcome::Win => 6,
    };

    return shape_score + outcome_score;
}
