use advent_of_code::get_aoc_input;
mod days;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // init
    dotenvy::dotenv()?;

    let input = get_aoc_input(1).await?;

    // day 1
    days::day_1::process(&input)?;

    Ok(())
}
