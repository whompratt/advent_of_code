use glam::IVec2;
use std::collections::HashMap;

// I struggled a lot with day-04. This solution was shamelessly stolen from Chris Biscardi, otherwise I'd still be struggling with it.

const OFFSETS: [[IVec2; 3]; 8] = [
    [IVec2::new(0, 1), IVec2::new(0, 2), IVec2::new(0, 3)],
    [IVec2::new(0, -1), IVec2::new(0, -2), IVec2::new(0, -3)],
    [IVec2::new(1, 1), IVec2::new(2, 2), IVec2::new(3, 3)],
    [IVec2::new(1, -1), IVec2::new(2, -2), IVec2::new(3, -3)],
    [IVec2::new(-1, 1), IVec2::new(-2, 2), IVec2::new(-3, 3)],
    [IVec2::new(-1, -1), IVec2::new(-2, -2), IVec2::new(-3, -3)],
    [IVec2::new(1, 0), IVec2::new(2, 0), IVec2::new(3, 0)],
    [IVec2::new(-1, 0), IVec2::new(-2, 0), IVec2::new(-3, 0)],
];

const MAS: [char; 3] = ['M', 'A', 'S'];

#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> miette::Result<String> {
    // Parse input string to HaskMap of x, y coordinates and the respective character
    let grid = input
        .lines()
        .enumerate() // Yield count and element from iterator
        .flat_map(|(y, row)| {
            // flat_map maps each element to an iterator, then yields the elements of _those_ iterators
            row.chars()
                .enumerate()
                .map(move |(x, character)| (IVec2::new(x as i32, y as i32), character))
        })
        .collect::<HashMap<IVec2, char>>();

    let result: usize = grid
        .iter()
        .filter(|(_, character)| **character == 'X') // Remove coordinates with non-x characters
        .map(|(coordinate, _)| {
            let count = OFFSETS
                .iter()
                .map(|offsets| {
                    // Iter the defined offsets
                    offsets
                        .iter()
                        .map(|offset| grid.get(&(coordinate + offset))) // Note that .get returns an Option<&V>, so non-existant coordinates are safe
                        .enumerate()
                        .all(|(index, character)| MAS.get(index) == character) // and filter out offsets that don't match 'M' 'A' 'S'
                })
                .filter(|b| *b)
                .count(); // Then count the remaining ones
            count
        })
        .sum();

    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test_log::test]
    fn test_process() -> miette::Result<()> {
        let input = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
        assert_eq!("18", process(input)?);
        Ok(())
    }
}
