use crate::util;
use regex::Regex;

pub async fn answer() -> Result<(u32, u32), anyhow::Error> {
    let input = util::get_input(3).await?;
    let pairs = convert(find_valid(&input)?);
    let part_1 = total(pairs);
    Ok((part_1, 1))
}

fn find_valid(input: &str) -> Result<Vec<(&str, &str)>, anyhow::Error> {
    let re = Regex::new(r"mul\((?<a>\d{1,3}),(?<b>\d{1,3})\)")?;
    let commands: Vec<(&str, &str)> = re
        .captures_iter(input)
        .map(|caps| {
            (
                caps.name("a").unwrap().as_str(),
                caps.name("b").unwrap().as_str(),
            )
        })
        .collect();

    Ok(commands)
}

fn convert(input: Vec<(&str, &str)>) -> Vec<(u32, u32)> {
    input
        .into_iter()
        .map(|(a, b)| (a.parse::<u32>().unwrap(), b.parse::<u32>().unwrap()))
        .collect()
}

fn total(pairs: Vec<(u32, u32)>) -> u32 {
    pairs.iter().fold(0, |acc, (a, b)| acc + a * b)
}
