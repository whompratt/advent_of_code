use std::{collections::BinaryHeap, iter::zip};

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let mut left: BinaryHeap<i32> = BinaryHeap::new();
    let mut right: BinaryHeap<i32> = BinaryHeap::new();

    for line in input.lines() {
        let numbers: Vec<&str> = line.split(r#"   "#).collect();
        left.push(numbers[0].parse::<i32>().unwrap());
        right.push(numbers[1].parse::<i32>().unwrap());
    }

    let result: i32 = zip(left.into_sorted_vec(), right.into_sorted_vec())
        .map(|(l, r)| (l - r).abs())
        .sum();

    return Ok(result.to_string());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "3   4
4   3
2   5
1   3
3   9
3   3";
        assert_eq!("11", process(input)?);
        Ok(())
    }
}
