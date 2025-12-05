use std::fs;

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
        for index in 0..improved_ranges.len() {
            let num1 = improved_ranges[index][0];
            let num2 = improved_ranges[index][1];
            if num1 <= *lower && *lower <= num2 {
                if num2 > *upper {
                    continue 'outer;
                }
                if num2 <= *upper {
                    improved_ranges[index][1] = *upper;
                    continue 'outer;
                }
            }
            if num1 <= *upper && *upper <= num2 {
                if num1 < *lower {
                    continue 'outer;
                }
                if num1 >= *lower {
                    improved_ranges[index][0] = *lower;
                    continue 'outer;
                }
            }
        }
        improved_ranges.push(vec![*lower, *upper]);
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
