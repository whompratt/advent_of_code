use std::fs;

fn main() {
    let input = fs::read_to_string("./data/day-02.txt").expect("Unable to read file");
    println!("{:?}", part_2(&input));
}

fn check_values(values: &[i32]) -> bool {
    let mut increasing = true;
    let mut decreasing = true;

    for window in values.windows(2) {
        if window[0].abs_diff(window[1]) > 3 {
            return false;
        }
        increasing &= window[0] < window[1];
        decreasing &= window[0] > window[1];
        if !(increasing ^ decreasing) {
            return false;
        }
    }
    true
}

fn parse_line(line: &str) -> Vec<i32> {
    line.split_whitespace()
        .map(|x| -> i32 { x.parse().unwrap() })
        .collect()
}

fn part_2(input: &str) -> usize {
    let lines = input.lines();
    let safe_count = lines
        .filter(|line| {
            let values = parse_line(line);
            check_values(&values)
                || (0..values.len()).any(|i| {
                    let mut values = values.clone();
                    values.remove(i);
                    check_values(&values)
                })
        })
        .count();

    return safe_count;
}

#[cfg(test)]
mod tests {
    use crate::part_2;

    #[test]
    fn test_part_2() {
        let input = r#"7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9"#;

        assert_eq!(part_2(&input), 4);
    }
}
