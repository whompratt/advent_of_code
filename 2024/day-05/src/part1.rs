use std::collections::HashMap;

use nom::{
    bytes::complete::tag,
    character::complete::{self, line_ending},
    multi::{fold_many1, separated_list1},
    sequence::{separated_pair, terminated},
    IResult,
};

// Another solution shamelessly stolen from Chris Biscardi.
// Made a good opportunity to learn some nom.

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let (input, rules) = terminated(parse_rules, line_ending)(input).unwrap();
    let (_, pages) = parse_pages(input).unwrap();

    // Vector of the index of valid updates in the `pages` vector
    let result: Vec<usize> = pages
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
                        return None;
                    }
                }
                checked_pages = &update[0..(checked_pages.len() + 1)];

                if let Some(page) = remaining_pages.first() {
                    current_page = *page;
                    remaining_pages = &remaining_pages[1..];
                }
            }

            Some(index)
        })
        .collect();

    let result: u32 = result
        .iter()
        .map(|index| {
            let middle_page = pages[*index].len() / 2;
            pages[*index][middle_page]
        })
        .sum();

    Ok(result.to_string())
}

fn parse_rules(input: &str) -> IResult<&str, HashMap<u32, Vec<u32>>> {
    // Parses input &str
    // Uses `nom` parser, so return type is an IResult, where the first element is the remainder
    // of the input &str that _wasn't_ parsed.
    // The second element is a HashMap with u32 keys (the page to lookup), and the value is a
    // vector of u32s that should come _after_ that page.
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
    // Parses input &str
    // Uses `nom` parser, so return type is an IResult, where the first element is the remainder
    // of the input &str that _wasn't_ parsed.
    // The second element is a Vector of Vector u32s, containing the page numbers.
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
        assert_eq!("143", process(input)?);
        Ok(())
    }
}
