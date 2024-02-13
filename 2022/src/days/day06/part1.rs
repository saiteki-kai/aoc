// https://adventofcode.com/2022/6
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

    #[test]
    fn test_examples() {
        assert_eq!(find_marker("bvwbjplbgvbhsrlpgdmjqwftvncz", 4), 5);
        assert_eq!(find_marker("nppdvjthqldpwncqszvftbrmjlhg", 4), 6);
        assert_eq!(find_marker("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 4), 10);
        assert_eq!(find_marker("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 4), 11);
    }
}
