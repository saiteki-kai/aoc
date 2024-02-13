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

    #[test]
    fn test_examples() {
        assert_eq!(find_marker("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 14), 19);
        assert_eq!(find_marker("bvwbjplbgvbhsrlpgdmjqwftvncz", 14), 23);
        assert_eq!(find_marker("nppdvjthqldpwncqszvftbrmjlhg", 14), 23);
        assert_eq!(find_marker("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 14), 29);
        assert_eq!(find_marker("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 14), 26);
    }
}
