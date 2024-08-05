// https://adventofcode.com/2022/day/6
//
// Day 6: Tuning Trouble - Part 1

use super::shared::find_marker;

pub fn solve() {
    let input_text = include_str!("../../../inputs/day06");
    println!("{}", input_text);

    let m = find_marker(input_text, 4);
    println!("{}", m);
}

#[cfg(test)]
mod tests {
    use super::find_marker;
    use test_case::test_case;

    #[test_case("bvwbjplbgvbhsrlpgdmjqwftvncz", 5)]
    #[test_case("nppdvjthqldpwncqszvftbrmjlhg", 6)]
    #[test_case("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 10)]
    #[test_case("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 11)]
    fn test_examples(input: &str, index: usize) {
        assert_eq!(find_marker(input, 4), index);
    }
}
