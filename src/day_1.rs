use crate::util;

pub async fn answer() -> Result<(u32, u32), anyhow::Error> {
    let input = util::get_input(1).await?;
    let mut split_input = groom(input);
    sort(&mut split_input);
    Ok((get_difference(&split_input), get_score(&split_input)))
}

fn _groom_pairs(input: String) -> Vec<(u32, u32)> {
    let pairs: Vec<(u32, u32)> = input
        .lines()
        .map(|line| {
            let numbers: Vec<u32> = line.split("   ").filter_map(|s| s.parse().ok()).collect();

            (numbers[0], numbers[1])
        })
        .collect();

    pairs
}

fn groom(input: String) -> (Vec<u32>, Vec<u32>) {
    input.lines().fold(
        (Vec::new(), Vec::new()),
        |(mut firsts, mut seconds), line| {
            let numbers: Vec<u32> = line.split("   ").filter_map(|s| s.parse().ok()).collect();

            firsts.push(numbers[0]);
            seconds.push(numbers[1]);

            (firsts, seconds)
        },
    )
}

fn sort(lists: &mut (Vec<u32>, Vec<u32>)) {
    lists.0.sort();
    lists.1.sort();
}

fn get_difference(lists: &(Vec<u32>, Vec<u32>)) -> u32 {
    lists.0.iter().zip(lists.1.iter()).fold(
        0,
        |acc, (&x, &y)| {
            if x > y {
                acc + (x - y)
            } else {
                acc + (y - x)
            }
        },
    )
}

fn get_score(lists: &(Vec<u32>, Vec<u32>)) -> u32 {
    lists
        .0
        .iter()
        .map(|&elem| (elem, lists.1.clone()))
        .fold(0, |acc, (x, y)| {
            acc + (x * y.iter().filter(|&n| *n == x).count() as u32)
        })
}
