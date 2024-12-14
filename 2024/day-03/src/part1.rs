use regex::Regex;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let re = Regex::new(r#"mul\((?P<first>[\d]{1,3}),(?P<second>[\d]{1,3})\)"#).unwrap();

    let mut pairs: Vec<(i32, i32)> = vec![];
    for (_, [first, second]) in re.captures_iter(input).map(|c| c.extract()) {
        pairs.push((
            first.parse::<i32>().unwrap(),
            second.parse::<i32>().unwrap(),
        ));
    }

    return Ok(pairs.iter().fold(0, |sum, e| sum + (e.0 * e.1)).to_string());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        assert_eq!("161", process(input)?);
        Ok(())
    }
}
