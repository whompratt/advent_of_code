use std::collections::HashMap;

use nom::{
    bytes::complete::tag,
    character::complete::{self, line_ending},
    multi::{fold_many1, separated_list1},
    sequence::{separated_pair, terminated},
    IResult,
};

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let (input, rules) = terminated(parse_rules, line_ending)(input).unwrap();
    let (_, updates) = parse_pages(input).unwrap();

    let bad_update_indexes: Vec<usize> = updates
        .iter()
        .enumerate()
        .filter_map(|(index, update)| {
            let mut current_page = update[0];
            let mut remaining_pages = &update[1..];
            let mut checked_pages = &update[..0];

            while checked_pages.len() < update.len() {
                if let Some(must_follow_pages) = rules.get(&current_page) {
                    if !must_follow_pages
                        .iter()
                        .all(|page| !checked_pages.contains(page))
                    {
                        return Some(index);
                    }
                }
                checked_pages = &update[0..(checked_pages.len() + 1)];

                if let Some(page) = remaining_pages.first() {
                    current_page = *page;
                    remaining_pages = &remaining_pages[1..];
                }
            }

            None
        })
        .collect();

    let fixed_updates: Vec<Vec<u32>> = bad_update_indexes
        .iter()
        .map(|update_index| {
            let mut fixed_pages: Vec<u32> = Vec::new();
            let bad_pages = &updates[*update_index];

            for page in bad_pages {
                let previous_pages = fixed_pages.clone();
                fixed_pages.push(*page);

                if let Some(must_follow_pages) = rules.get(page) {
                    for previous_page in previous_pages.iter() {
                        if must_follow_pages.contains(previous_page) {
                            let bad_page_index = fixed_pages
                                .iter()
                                .position(|bad_page| bad_page == previous_page)
                                .unwrap();
                            let removed_page = fixed_pages.remove(bad_page_index);
                            fixed_pages.push(removed_page);
                        }
                    }
                }
            }

            fixed_pages
        })
        .collect();

    let result: u32 = fixed_updates
        .iter()
        .map(|update| {
            let middle_page = update.len() / 2;
            update[middle_page]
        })
        .sum();

    Ok(result.to_string())
    // Ok("".to_string())
}

fn parse_rules(input: &str) -> IResult<&str, HashMap<u32, Vec<u32>>> {
    fold_many1(
        terminated(
            separated_pair(complete::u32, tag("|"), complete::u32),
            line_ending,
        ),
        HashMap::default,
        |mut acc: HashMap<u32, Vec<u32>>, (page, after)| {
            acc.entry(page)
                .and_modify(|afters| {
                    afters.push(after);
                })
                .or_insert(vec![after]);
            acc
        },
    )(input)
}

fn parse_pages(input: &str) -> IResult<&str, Vec<Vec<u32>>> {
    separated_list1(line_ending, separated_list1(tag(","), complete::u32))(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";
        assert_eq!("123", process(input)?);
        Ok(())
    }
}
