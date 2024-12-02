use std::fs;

fn main() {
    let input = fs::read_to_string("./data/day-02.txt").expect("Unable to read file");
    let safe_count = part_1(&input);

    println!("Safe count: {}", safe_count);
}

enum Direction {
    Ascending,
    Descending,
}

impl Direction {
    fn is_valid_next(&self, current: &i32, next: &i32) -> bool {
        match self {
            Direction::Ascending => (1..4).contains(&(next - current)),
            Direction::Descending => (1..4).contains(&(current - next)),
        }
    }
}

fn part_1(input: &str) -> i32 {
    let mut safe_count: i32 = 0;

    for line in input.lines() {
        let row: Vec<i32> = line
            .split_whitespace()
            .map(|x| -> i32 { x.parse().unwrap() })
            .collect();

        let first_pair: i32 = row[1] - row[0];

        let direction = if (1..4).contains(&first_pair) {
            Direction::Ascending
        } else if (1..4).contains(&-first_pair) {
            Direction::Descending
        } else {
            continue;
        };

        if row[..]
            .iter()
            .zip(&row[1..])
            .all(|(current, next)| direction.is_valid_next(current, next))
        {
            safe_count += 1;
        };
    }

    return safe_count;
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

        assert_eq!(part_1(input), 2);
    }
}
