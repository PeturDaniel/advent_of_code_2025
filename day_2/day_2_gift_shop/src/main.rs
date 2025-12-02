use std::fs;

fn repeated_sequence(low: i64, high: i64) -> i64 {
    let mut count: i64 = 0;
    for number in low..=high {
        let number_str = number.to_string();
        let length = number_str.len();
        if length % 2 != 0 {
            continue;
        }
        let middle = length / 2;

        let left: i64 = number_str[..middle].parse().unwrap_or(0);
        let right: i64 = number_str[middle..].parse().unwrap_or(0);
        if left == right {
            count += number;
        }
    }
    count
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let contents = fs::read_to_string("../input.txt")?;

    let mut sum: i64 = 0;

    for range in contents.split(',').map(str::trim).filter(|s| !s.is_empty()) {
        let (low_str, high_str) = range
            .split_once('-')
            .ok_or_else(|| format!("Invalid range format: `{}`", range))?;

        let low: i64 = low_str.parse()?;
        let high: i64 = high_str.parse()?;

        sum += repeated_sequence(low, high);
    }

    println!("Final sum of IDs: {sum}");

    Ok(())
}
