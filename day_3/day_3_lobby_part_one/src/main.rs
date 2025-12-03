use std::fs::File;
use std::io::{BufRead, BufReader};

fn count_two_highest(line: String) -> i64 {
    let bytes = line.as_bytes();

    if bytes.len() == 0 {
        return 0;
    }

    let mut highest = 0;
    let mut highest_index = 0;

    for index in 0..bytes.len() - 1 {
        let b = bytes[index];
        let curr = b - b'0';

        if curr == 9 {
            highest = 9;
            highest_index = index;
            break;
        }

        if curr > highest {
            highest = curr;
            highest_index = index;
        }
    }

    let mut sec_highest = 0;

    for index in highest_index + 1..bytes.len() {
        let b = bytes[index];
        let curr = b - b'0';

        if curr == 9 {
            sec_highest = curr;
            break;
        }
        if curr > sec_highest {
            sec_highest = curr;
        }
    }

    let rtn_num: i64 = (highest as i64 * 10 + sec_highest as i64) as i64;
    rtn_num
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open("../input.txt")?;
    let reader = BufReader::new(file);
    let mut count: i64 = 0;
    for line_result in reader.lines() {
        let line = line_result?;
        count += count_two_highest(line);
    }

    println!("Total joltage: {}", count);

    Ok(())
}
