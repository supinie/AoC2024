use reqwest::{
    header::{HeaderMap, HeaderValue, COOKIE},
    Client,
};
use std::fs;

pub async fn get_input(day: usize) -> Result<String, anyhow::Error> {
    let client = Client::new();

    let endpoint = format!("https://adventofcode.com/2024/day/{day}/input");

    let mut session = fs::read_to_string("session")
        .expect("Session token should exist.\nPlease get it from your browser and store it in `session` file now.");

    // add check its newline thats removed
    _ = session.pop();
    let cookie = HeaderValue::from_str(&format!("session={session}"))?;
    let mut headers = HeaderMap::new();
    headers.insert(COOKIE, cookie);

    let body = client
        .get(endpoint)
        .headers(headers)
        .send()
        .await?
        .text()
        .await?;

    Ok(body.to_owned())
}

// pub async fn send_answer(day: usize, answer:
