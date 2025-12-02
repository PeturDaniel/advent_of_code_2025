use std::fs;

fn repeated_sequence(low: i64, high: i64) -> i64 {
    let mut count: i64 = 0;
    'outer: for number in low..=high {
        let number_str = number.to_string();
        let length = number_str.len();
        let middle = length / 2;

        for index in 0..middle {
            if length % (index + 1) != 0 {
                continue;
            }
            let num_of_sub_seq = length / (1 + index);
            let mut num_arr = vec![];
            let mut initial = 0;
            let numbers: Vec<char> = number_str.chars().collect();
            let seq_length = index + 1;
            for _ in 0..num_of_sub_seq {
                let sub_seq: String = numbers[initial..initial + seq_length].iter().collect();
                num_arr.push(sub_seq);
                initial += seq_length;
            }
            let equal = num_arr
                .first()
                .map(|first| num_arr.iter().all(|p| p == first))
                .unwrap_or(true);
            if equal {
                count += number;
                continue 'outer;
            }
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
