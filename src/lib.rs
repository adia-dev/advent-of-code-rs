use std::env;

use reqwest::header::{HeaderMap, COOKIE};

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub async fn get_aoc_input(day: u8) -> Result<String> {
    dotenvy::dotenv()?;

    let session_cookie = env::var("SESSION_COOKIE").expect("Please fill the SESSION_COOKIE variable in a .env file or run the executable with: SESSION_COOKIE=\"...\" <executable>");

    let mut headers = HeaderMap::new();
    headers.insert(COOKIE, session_cookie.parse().unwrap());
    let client = reqwest::Client::new();

    let res = client
        .get(format!("https://adventofcode.com/2022/day/{}/input", day))
        .headers(headers)
        .send()
        .await?;

    let body = res.text().await?;

    Ok(body)
}
