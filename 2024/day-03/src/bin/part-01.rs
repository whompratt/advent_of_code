use std::fs;

use regex::Regex;

fn main() {
    let input = fs::read_to_string("./data/day-03.txt").expect("Unable to read file");
    let result = part_1(&input);

    println!("{}", result);
}

fn part_1(input: &str) -> i32 {
    let re = Regex::new(r#"mul\((?P<first>[\d]{1,3}),(?P<second>[\d]{1,3})\)"#).unwrap();

    let mut result: i32 = 0;
    for (_, [first, second]) in re.captures_iter(input).map(|c| c.extract()) {
        let mul = first.parse::<i32>().unwrap() * second.parse::<i32>().unwrap();
        result += mul;
    }

    return result;
}

#[cfg(test)]
mod tests {
    use crate::part_1;

    #[test]
    fn test_part_1() {
        let input = r#"7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9"#;

        assert_eq!(part_1(input), 0);
    }
}
