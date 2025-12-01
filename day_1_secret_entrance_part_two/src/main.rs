use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);
    let modulus: i64 = 100;
    let mut dial: i64 = 50;
    let mut counter = 0;
    for line_result in reader.lines() {
        let line = line_result?;
        let (dir_str, num_str) = line.split_at(1);
        let dir = dir_str.chars().next().unwrap();
        let amount: i64 = num_str.parse().unwrap();

        match dir {
            'R' => {
                counter += (dial + amount) / modulus;
                dial = (dial + amount) % modulus;
            }
            'L' => {
                let delta = dial - amount;
                if delta < 0 && dial != 0 {
                    counter += 1;
                }
                let extra = (delta / modulus).abs();
                dial = (((dial - amount) % modulus) + modulus) % modulus;
                if dial == 0 {
                    counter += 1;
                }
                if dial == 0 && extra > 0 {
                    counter -= 1;
                }
                counter += extra;
            }
            _ => {
                println!("Value not R or L");
            }
        }
    }

    println!("Final tally is: {counter}");
    Ok(())
}
