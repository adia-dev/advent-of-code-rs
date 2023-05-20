use std::{env, fs};

use reqwest::header::{HeaderMap, COOKIE};

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub async fn get_aoc_input(day: u8) -> Result<String> {
    if !std::path::Path::new(".cache/puzzles/").exists() {
        fs::create_dir_all(".cache/puzzles/")?;
    }

    let cached_puzzle_path: String = format!(".cache/puzzles/day_{day}.txt");

    if std::path::Path::new(&cached_puzzle_path).exists() {
        println!("Using the cached puzzle for day {day} !");

        let puzzle = fs::read_to_string(cached_puzzle_path)
            .expect("An error occurred while reading a file that supposedly exists.");
        return Ok(puzzle);
    }

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

    fs::write(&cached_puzzle_path, &body).expect("Unable to write the puzzle input into the cache");

    Ok(body)
}
