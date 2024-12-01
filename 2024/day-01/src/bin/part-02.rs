use std::{
    collections::{BinaryHeap, HashMap},
    fs,
};

fn main() {
    let input = fs::read_to_string("./data/day-01.txt").expect("Unable to read file");
    let similarity = part_2(&input);

    println!("Similarity: {}", similarity);
}

fn part_2(input: &str) -> i32 {
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

    return running;
}

#[cfg(test)]
mod tests {
    use crate::part_2;
    #[test]
    fn test_part_2() {
        let input = r#"3   4
4   3
2   5
1   3
3   9
3   3"#;

        assert_eq!(part_2(input), 31);
    }
}
