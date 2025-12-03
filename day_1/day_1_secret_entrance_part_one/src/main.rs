use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);
    let modulus = 100;
    let mut dial = 50;
    let mut counter = 0;
    for line_result in reader.lines() {
        let line = line_result?;
        let (dir_str, num_str) = line.split_at(1);
        let dir = dir_str.chars().next().unwrap();
        let amount: i32 = num_str.parse().unwrap();

        match dir {
            'R' => {
                dial = (dial + amount) % modulus;
                if dial == 0 {
                    counter += 1;
                }
            }
            'L' => {
                dial = (dial - amount + modulus) % modulus;
                if dial == 0 {
                    counter += 1;
                }
            }
            _ => {
                println!("Value not R or L");
            }
        }
    }

    println!("Final tally is: {counter}");
    Ok(())
}
