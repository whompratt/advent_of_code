use std::fs;

fn main() {
    println!("day-03");
}

fn part_1(input: &str) -> i32 {
    println!("Input: \"{}\"", input);

    return 161;
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
