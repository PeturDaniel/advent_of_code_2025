use std::fs;

fn part_one(input: Vec<Vec<String>>) -> i64 {
    let mut total: i64 = 0;

    for line in 0..input[0].len() {
        let mut count: i64 = input[0][line].parse().unwrap_or(0);
        let mut index = input.len() - 2;
        let mut multiply = false;
        let mut add = false;
        if input[input.len() - 1][line] == "*" {
            multiply = true;
        }
        if input[input.len() - 1][line] == "+" {
            add = true;
        }
        while index > 0 {
            if multiply {
                count *= input[index][line].parse().unwrap_or(0);
            }
            if add {
                count += input[index][line].parse().unwrap_or(0);
            }
            index -= 1;
        }
        total += count;
    }

    total
}

fn part_two(input: String) -> u64 {
    let mut total: u64 = 0;

    let lines: Vec<&str> = input.lines().collect();
    let operator_line = lines.len() - 1;

    let row_length = lines[0].len();

    let mut curr_operator: String = String::new();

    let mut count: u64 = 0;
    'outer: for i in 0..row_length {
        let mut possible_operator: String = String::new();
        possible_operator.push(lines[operator_line].as_bytes()[i] as char);
        if !possible_operator.is_empty() {
            if possible_operator == "*" {
                curr_operator = possible_operator.clone();
                count = 1;
            }
            if possible_operator == "+" {
                curr_operator = possible_operator.clone();
                count = 0;
            }
        }
        let mut num = String::new();
        for j in 0..operator_line {
            let curr_line = lines[j].as_bytes();
            let curr_char = curr_line[i];
            num.push(curr_char as char);
        }
        println!("{}", num.trim().parse().unwrap_or(0));
        println!("Current operator: {}", curr_operator);
        let final_num: u64 = num.trim().parse().unwrap_or(0);
        if final_num == 0 {
            total += count;
            continue 'outer;
        }
        if curr_operator == "*" && final_num != 0 {
            count *= final_num;
        }
        if curr_operator == "+" && final_num != 0 {
            count += final_num;
        }
    }

    total += count;

    total
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = fs::read_to_string("./input.txt")?;
    let grid: Vec<Vec<String>> = input
        .lines()
        .map(|line| line.split_whitespace().map(|s| s.to_string()).collect())
        .collect();

    println!("Part one total: {}", part_one(grid));

    let input2 = fs::read_to_string("./input.txt")?;

    println!("Part two total: {}", part_two(input2));

    Ok(())
}
