// https://adventofcode.com/2022/6
//
// Day 6: Tuning Trouble - Part 2

use super::shared::find_marker;

pub fn solve() {
    let input_text = include_str!("../../../inputs/day06");
    println!("{}", input_text);

    let m = find_marker(input_text, 14);
    println!("{}", m);
}

#[cfg(test)]
mod tests {
    use super::find_marker;
    use test_case::test_case;

    #[test_case("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 19)]
    #[test_case("bvwbjplbgvbhsrlpgdmjqwftvncz", 23)]
    #[test_case("nppdvjthqldpwncqszvftbrmjlhg", 23)]
    #[test_case("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 29)]
    #[test_case("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 26)]
    fn test_find_marker(input: &str, index: usize) {
        assert_eq!(find_marker(input, 14), index);
    }
}
