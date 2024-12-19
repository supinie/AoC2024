use crate::util;
use regex::Regex;

pub async fn answer() -> Result<(u32, u32), anyhow::Error> {
    let input = util::get_input(3).await?;
    let part_1 = total(convert(find_valid(&input)?));
    let part_2 = total(strip_donts(find_valid_extra(&input)?));
    Ok((part_1, part_2))
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

fn find_valid_extra(input: &str) -> Result<Vec<(&str, &str)>, anyhow::Error> {
    let re = Regex::new(r"(?<command>do\(\)|don't\(\))|mul\((?<a>\d{1,3}),(?<b>\d{1,3})\)")?;
    let commands: Vec<(&str, &str)> = re
        .captures_iter(input)
        .map(|caps| {
            if let Some(a) = caps.name("a")
                && let Some(b) = caps.name("b")
            {
                (a.as_str(), b.as_str())
            } else {
                (caps.name("command").unwrap().as_str(), "null")
            }
        })
        .collect();

    Ok(commands)
}

fn strip_donts(input: Vec<(&str, &str)>) -> Vec<(u32, u32)> {
    let mut last_order = "do()";
    input
        .into_iter()
        .map(|(a, b)| {
            if b == "null" {
                last_order = a;
                return (0, 0);
            }

            if last_order == "don't()" {
                (0, 0)
            } else {
                (a.parse::<u32>().unwrap(), b.parse::<u32>().unwrap())
            }
        })
        .collect()
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
