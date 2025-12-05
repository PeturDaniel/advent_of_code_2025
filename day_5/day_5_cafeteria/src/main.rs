use std::{collections::HashSet, fs};

fn part_one(ranges: Vec<(i64, i64)>, ids: Vec<i64>) -> u64 {
    let mut count: u64 = 0;
    'outer: for id in ids {
        for (lower_bound, upper_bound) in &ranges {
            if *lower_bound <= id && id <= *upper_bound {
                count += 1;
                continue 'outer;
            }
        }
    }
    count
}

fn part_two(mut ranges: Vec<(i64, i64)>) -> u64 {
    let mut improved_ranges: Vec<Vec<i64>> = vec![];

    println!("Before sort: {:#?}", ranges);
    ranges.sort();
    println!("After sort: {:#?}", ranges);

    'outer: for (lower_bound, upper_bound) in &ranges {
        let lower = lower_bound;
        let upper = upper_bound;
        let mut changed = false;
        for index in 0..improved_ranges.len() {
            let mut num1 = improved_ranges[index][0];
            let mut num2 = improved_ranges[index][1];
            if num1 <= *lower && *lower <= num2 {
                if num2 > *upper {
                    changed = true;
                }
                if num2 <= *upper {
                    num2 = *upper;
                    changed = true;
                }
            }
            if num1 <= *upper && *upper <= num2 {
                if num1 < *lower {
                    changed = true;
                }
                if num1 >= *lower {
                    num1 = *lower;
                    changed = true;
                }
            }
            if changed {
                improved_ranges[index][0] = num1;
                improved_ranges[index][1] = num2;
                continue 'outer;
            }
        }
        if !changed {
            improved_ranges.push(vec![*lower, *upper]);
        }
    }

    println!("Improved ranges: {:#?}", improved_ranges);

    let mut count: u64 = 0;
    for index in 0..improved_ranges.len() {
        count += improved_ranges[index][1] as u64 - improved_ranges[index][0] as u64 + 1;
    }

    count
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = fs::read_to_string("./input.txt")?;

    let mut ranges: Vec<(i64, i64)> = vec![];
    let mut ids: Vec<i64> = vec![];

    let mut getting_ranges = true;

    for line_result in file.lines() {
        let line = line_result.trim();

        if line.is_empty() {
            getting_ranges = false;
            continue;
        }

        if getting_ranges {
            let (lower, upper) = line.split_once('-').ok_or("")?;

            let lower_bound = lower.parse()?;
            let upper_bound = upper.parse()?;
            ranges.push((lower_bound, upper_bound));
        } else {
            let id: i64 = line.parse()?;
            ids.push(id);
        }
    }

    let p1 = part_one(ranges.clone(), ids);
    println!("Fresh ingredients: {}", p1);
    let p2 = part_two(ranges);
    println!("Fresh ingredients: {}", p2);

    Ok(())
}
