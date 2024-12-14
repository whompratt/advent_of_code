use regex::Regex;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let re =
        Regex::new(r#"(mul\((?<first>[0-9]+),(?<second>[0-9]+)\))|(?<dt>don't\(\))|(?<d>do\(\))"#)
            .unwrap();

    let mut pairs: Vec<(i32, i32)> = vec![];
    let mut flag: bool = true;

    for captures in re.captures_iter(input) {
        if let Some(_) = captures.name("d") {
            flag = true;
        } else if let Some(_) = captures.name("dt") {
            flag = false;
        } else if let (Some(first), Some(second)) =
            (captures.name("first"), captures.name("second"))
        {
            if flag {
                pairs.push((
                    first.as_str().parse::<i32>().unwrap(),
                    second.as_str().parse::<i32>().unwrap(),
                ));
            }
        }
    }

    let sum = pairs.iter().fold(0, |sum, e| sum + (e.0 * e.1));

    return Ok(sum.to_string());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        assert_eq!("48", process(input)?);
        Ok(())
    }
}
