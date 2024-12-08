use std::fs;

use regex::Regex;

fn main() {
    let input = fs::read_to_string("./data/day-03.txt").expect("Unable to read file");
    let result = part_1(&input);

    println!("{}", result);
}

fn part_1(input: &str) -> i32 {
    let re = Regex::new(r#"mul\((?P<first>[\d]{1,3}),(?P<second>[\d]{1,3})\)"#).unwrap();

    let mut pairs: Vec<(i32, i32)> = vec![];
    for (_, [first, second]) in re.captures_iter(input).map(|c| c.extract()) {
        pairs.push((
            first.parse::<i32>().unwrap(),
            second.parse::<i32>().unwrap(),
        ));
    }

    return pairs.iter().fold(0, |sum, e| sum + (e.0 * e.1));
}

#[cfg(test)]
mod tests {
    use crate::part_1;

    #[test]
    fn test_part_1() {
        let input = r#"xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"#;
        let expected: i32 = 161;
        let actual: i32 = part_1(&input);

        assert_eq!(actual, expected);
    }
}
