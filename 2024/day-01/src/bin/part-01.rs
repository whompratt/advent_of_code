use std::{collections::BinaryHeap, fs, iter::zip};

fn main() {
    let input = fs::read_to_string("./data/day-01.txt").expect("Unable to read file");
    let offset = part_1(&input);

    println!("Offset: {}", offset);
}

fn part_1(input: &str) -> i32 {
    let mut heap_left: BinaryHeap<i32> = BinaryHeap::new();
    let mut heap_right: BinaryHeap<i32> = BinaryHeap::new();

    for line in input.lines() {
        let numbers: Vec<&str> = line.split(r#"   "#).collect();
        heap_left.push(numbers[0].parse::<i32>().unwrap());
        heap_right.push(numbers[1].parse::<i32>().unwrap());
    }

    let (vec_left, vec_right) = (heap_left.into_sorted_vec(), heap_right.into_sorted_vec());

    let mut offset: i32 = 0;
    for (left, right) in zip(vec_left, vec_right) {
        offset = offset + (left - right).abs();
    }

    return offset;
}

#[cfg(test)]
mod tests {
    use crate::part_1;

    #[test]
    fn test_part_1() {
        let input = r#"3   4
4   3
2   5
1   3
3   9
3   3"#;

        assert_eq!(part_1(input), 11);
    }
}
