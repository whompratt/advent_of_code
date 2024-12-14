use glam::IVec2;
use std::collections::HashMap;

const OFFSETS: [[IVec2; 2]; 4] = [
    [IVec2::new(-1, -1), IVec2::new(1, 1)],
    [IVec2::new(1, 1), IVec2::new(-1, -1)],
    [IVec2::new(1, -1), IVec2::new(-1, 1)],
    [IVec2::new(-1, 1), IVec2::new(1, -1)],
];

const MAS: [char; 2] = ['M', 'S'];

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let grid = input
        .lines()
        .enumerate()
        .flat_map(|(y, row)| {
            row.chars()
                .enumerate()
                .map(move |(x, character)| (IVec2::new(x as i32, y as i32), character))
        })
        .collect::<HashMap<IVec2, char>>();

    let result: usize = grid
        .iter()
        .filter(|(_, character)| **character == 'A')
        .filter(|(coordinate, _)| {
            OFFSETS
                .iter()
                .map(|offsets| {
                    // Iter the defined offsets
                    offsets
                        .iter()
                        .map(|offset| grid.get(&(*coordinate + offset)))
                        .enumerate()
                        .all(|(index, character)| MAS.get(index) == character)
                })
                .filter(|b| *b)
                .count()
                == 2
        })
        .count();

    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
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
        assert_eq!("9", process(input)?);
        Ok(())
    }
}
