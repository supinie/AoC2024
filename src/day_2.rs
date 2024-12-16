use crate::util;

pub async fn answer() -> Result<(u32, u32), anyhow::Error> {
    let input = groom(util::get_input(2).await?);
    let safe_reports = test_reports(&input);
    let safe_dampened = test_with_dampening(&input);
    Ok((safe_reports, safe_dampened))
}

fn groom(input: String) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|line| line.split(" ").filter_map(|s| s.parse().ok()).collect())
        .collect()
}

fn increasing(report: &[i32]) -> u32 {
    report
        .windows(2)
        .map(|window| if window[0] >= window[1] { 0 } else { 1 })
        .product()
}

fn decreasing(report: &[i32]) -> u32 {
    report
        .windows(2)
        .map(|window| if window[0] <= window[1] { 0 } else { 1 })
        .product()
}

fn difference(report: &[i32]) -> u32 {
    report
        .windows(2)
        .map(|window| {
            if (window[0] - window[1]).abs() > 3 {
                0
            } else {
                1
            }
        })
        .product()
}

fn test_reports(reports: &[Vec<i32>]) -> u32 {
    reports.iter().fold(0, |acc, report| {
        acc + ((increasing(report) + decreasing(report)) * difference(report))
    })
}

fn test_with_dampening(reports: &[Vec<i32>]) -> u32 {
    reports.iter().fold(0, |acc, report| {
        acc + (0..report.len())
            .map(|i| {
                let mut temp_report = report.clone();
                temp_report.remove(i);

                (increasing(&temp_report) + decreasing(&temp_report)) * difference(&temp_report)
            })
            .any(|x| x != 0) as u32
    })
}
