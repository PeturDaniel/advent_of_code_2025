use std::fs::File;
use std::io::{BufRead, BufReader};

fn count_twelve_highest(line: String) -> i64 {
    let bytes = line.as_bytes();
    let mut rtn_num: i64 = 0;

    if bytes.len() == 0 {
        return 0;
    }

    let mut highest_index = 0;

    for i in 0..12 {
        let mut highest = 0;
        for index in highest_index..bytes.len() - 11 + i {
            let b = bytes[index];
            let curr = b - b'0';

            if curr > highest {
                highest = curr;
                highest_index = index + 1;
            }
        }
        rtn_num = rtn_num * 10 + highest as i64;
    }
    rtn_num
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open("../input.txt")?;
    let reader = BufReader::new(file);
    let mut count: i64 = 0;
    for line_result in reader.lines() {
        let line = line_result?;
        count += count_twelve_highest(line);
    }

    println!("Total joltage: {}", count);

    Ok(())
}
