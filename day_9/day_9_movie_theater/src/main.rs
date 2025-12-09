use std::fs;

fn part_one(input: &Vec<(i64, i64)>) -> i64 {
    let mut area: i64 = 0;
    for a in 0..input.len() - 1 {
        for b in a + 1..input.len() {
            let delta_x = (input[a].0 - input[b].0).abs() + 1;
            let delta_y = (input[a].1 - input[b].1).abs() + 1;
            let possible_area = delta_x * delta_y;
            if possible_area > area {
                area = possible_area;
            }
        }
    }

    area
}

fn part_two(input: &mut Vec<(i64, i64, i64)>) -> i64 {
    let mut area: i64 = 0;
    let mut walls: Vec<(i64, i64, i64, i64)> = vec![];
    println!("Input before sort: {:#?}", input);
    input.sort_unstable_by_key(|&(x, y, _)| (y, x));
    println!("Input after sort: {:#?}", input);

    'outer: for a in 0..input.len() - 1 {
        'inner: for b in 0..input.len() {
            if input[a].2 == 2 {
                continue 'outer;
            }
            if input[b].2 == 2 {
                continue 'inner;
            }
            if input[a].0 == input[b].0 {}
        }
    }

    area
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = fs::read_to_string("./test-input.txt")?;

    let mut coordinates: Vec<(i64, i64)> = vec![];
    let mut coordinates_2: Vec<(i64, i64, i64)> = vec![];

    for line in input.lines() {
        let (x_str, y_str) = line.split_once(",").ok_or_else(|| "")?;
        let x: i64 = x_str.parse().unwrap();
        let y: i64 = y_str.parse().unwrap();
        coordinates.push((x, y));
        coordinates_2.push((x, y, 0));
    }

    println!("Biggest area for part one: {}", part_one(&coordinates));
    println!(
        "Biggest area for part two: {}",
        part_two(&mut coordinates_2)
    );

    Ok(())
}
