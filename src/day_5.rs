use crate::util;

pub async fn answer() -> Result<(u32, u32), anyhow::Error> {
    let (rules, updates) = groom(util::get_input(5).await?);
    let valid_updates = test_updates(updates, rules);
    println!("{:?}", valid_updates);
    let part_1 = sum_middles(valid_updates);
    Ok((part_1, 1))
}

fn groom(input: String) -> (Vec<(u32, u32)>, Vec<Vec<u32>>) {
    let halves: Vec<&str> = input.split("\n\n").collect();
    let rules = halves[0]
        .lines()
        .map(|line| {
            let pairs: Vec<&str> = line.split("|").collect();
            (
                pairs[0].parse::<u32>().unwrap(),
                pairs[1].parse::<u32>().unwrap(),
            )
        })
        .collect();
    let updates = halves[1]
        .lines()
        .map(|line| line.split(",").map(|n| n.parse::<u32>().unwrap()).collect())
        .collect();

    (rules, updates)
}

fn test_updates(updates: Vec<Vec<u32>>, rules: Vec<(u32, u32)>) -> Vec<Vec<u32>> {
    updates
        .iter()
        .map(|update| {
            let mut valid = true;
            rules.iter().for_each(|rule| {
                if let Some(i) = update.iter().position(|&x| x == rule.0)
                    && let Some(j) = update.iter().position(|&y| y == rule.1)
                    && i > j
                {
                    valid = false;
                }
            });
            if valid {
                update.clone()
            } else {
                vec![0u32]
            }
        })
        .collect()
}

fn sum_middles(valid_updates: Vec<Vec<u32>>) -> u32 {
    valid_updates.iter().fold(0, |acc, updates| {
        println!("{:?}", updates);
        println!("{:?}", updates[((updates.len() + 1) / 2) - 1]);
        acc + updates[((updates.len() + 1) / 2) - 1]
    })
}
