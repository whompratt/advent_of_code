use std::fs;

use regex::Regex;

fn main() {
    let input = fs::read_to_string("./data/day-03.txt").expect("Unable to read file");
    let result = part_2(&input);

    println!("{}", result);
}

fn part_2(input: &str) -> i32 {
    let re = Regex::new(r#"(mul\((?<op1>[0-9]+),(?<op2>[0-9]+)\))|(?<dt>don't\(\))|(?<d>do\(\))"#)
        .unwrap();

    let mut pairs: Vec<(i32, i32)> = vec![];
    let mut flag: bool = true;

    for captures in re.captures_iter(input) {
        if let Some(_) = captures.name("d") {
            flag = true;
        } else if let Some(_) = captures.name("dt") {
            flag = false;
        } else if let (Some(op1), Some(op2)) = (captures.name("op1"), captures.name("op2")) {
            if flag {
                pairs.push((
                    op1.as_str().parse::<i32>().unwrap(),
                    op2.as_str().parse::<i32>().unwrap(),
                ));
            }
        }
    }

    return pairs.iter().fold(0, |sum, e| sum + (e.0 * e.1));
}

#[cfg(test)]
mod tests {
    use crate::part_2;

    #[test]
    fn test_part_2() {
        let input = r#"xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"#;
        let expected: i32 = 48;
        let actual: i32 = part_2(&input);

        assert_eq!(actual, expected);
    }
}
