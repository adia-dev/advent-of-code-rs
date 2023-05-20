
use advent_of_code::Result;

pub fn process(input: &str) -> Result<()> {
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
