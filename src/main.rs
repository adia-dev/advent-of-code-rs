use advent_of_code::get_aoc_input;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // init
    dotenvy::dotenv()?;

    let input = get_aoc_input(1).await?;
    let calories: Vec<&str> = input.split("\n").collect();
    let mut groups: Vec<i32> = Vec::new();
    let mut current = 0;

    for c in calories.into_iter() {
        if c.len() > 0 {
            current += c.parse::<i32>().unwrap();
        } else {
            groups.push(current);
            current = 0;
        }
    }

    groups.sort();

    let max = groups.iter().max().unwrap();

    let top_three: i32 = groups.iter().rev().take(3).sum();

    println!("1: {max}");
    println!("2: {top_three}");

    Ok(())
}
