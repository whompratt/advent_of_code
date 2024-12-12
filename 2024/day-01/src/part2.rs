use std::collections::{BinaryHeap, HashMap};

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let mut heap_left: BinaryHeap<i32> = BinaryHeap::new();
    let mut heap_right: BinaryHeap<i32> = BinaryHeap::new();

    for line in input.lines() {
        let numbers: Vec<&str> = line.split(r#"   "#).collect();
        heap_left.push(numbers[0].parse::<i32>().unwrap());
        heap_right.push(numbers[1].parse::<i32>().unwrap());
    }

    let (vec_left, vec_right) = (heap_left.into_sorted_vec(), heap_right.into_sorted_vec());
    let mut counts: HashMap<i32, i32> = HashMap::new();

    for right in vec_right {
        *counts.entry(right).or_insert(0) += 1;
    }

    let mut running: i32 = 0;
    for left in vec_left {
        running += left * *counts.entry(left).or_insert(0);
    }

    return Ok(format!("{}", running));
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
        assert_eq!("31", process(input)?);
        Ok(())
    }
}
