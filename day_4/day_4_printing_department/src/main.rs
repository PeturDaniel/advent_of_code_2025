use std::fs::File;
use std::io::{BufRead, BufReader};

fn count_line(line: &Vec<char>, index: usize) -> u64 {
    let mut count: u64 = 0;
    if index != 0 {
        if line[index - 1] == '@' {
            count += 1;
        }
    }
    if index != line.len() - 1 {
        if line[index + 1] == '@' {
            count += 1;
        }
    }
    if line[index] == '@' {
        count += 1;
    }

    count
}

fn part_one(grid: Vec<Vec<char>>) -> u64 {
    let mut count: u64 = 0;

    for outer_index in 0..grid.len() {
        for inner_index in 0..grid[outer_index].len() {
            if grid[outer_index][inner_index] == '.' {
                continue;
            }
            let mut neighbors: u64 = 0;
            if outer_index != 0 {
                neighbors += count_line(&grid[outer_index - 1], inner_index);
            }
            if outer_index != grid.len() - 1 {
                neighbors += count_line(&grid[outer_index + 1], inner_index);
            }
            if inner_index != 0 {
                if grid[outer_index][inner_index - 1] == '@' {
                    neighbors += 1;
                }
            }
            if inner_index != grid[outer_index].len() - 1 {
                if grid[outer_index][inner_index + 1] == '@' {
                    neighbors += 1;
                }
            }
            if neighbors < 4 {
                count += 1;
            }
        }
    }

    count
}

fn part_two(mut grid: Vec<Vec<char>>) -> u64 {
    let mut total: u64 = 0;
    let mut used: Vec<(usize, usize)> = vec![];
    let mut going = true;
    while going {
        let mut count: u64 = 0;
        for outer_index in 0..grid.len() {
            for inner_index in 0..grid[outer_index].len() {
                if grid[outer_index][inner_index] == '.' {
                    continue;
                }
                let mut neighbors: u64 = 0;
                if outer_index != 0 {
                    neighbors += count_line(&grid[outer_index - 1], inner_index);
                }
                if outer_index != grid.len() - 1 {
                    neighbors += count_line(&grid[outer_index + 1], inner_index);
                }
                if inner_index != 0 {
                    if grid[outer_index][inner_index - 1] == '@' {
                        neighbors += 1;
                    }
                }
                if inner_index != grid[outer_index].len() - 1 {
                    if grid[outer_index][inner_index + 1] == '@' {
                        neighbors += 1;
                    }
                }
                if neighbors < 4 {
                    count += 1;
                    used.push((outer_index, inner_index));
                }
            }
        }
        if count == 0 {
            going = false;
        }
        total += count;
        for (outer, inner) in &used {
            grid[*outer][*inner] = '.';
        }
        used.clear();
    }

    total
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open("./input.txt")?;
    let reader = BufReader::new(file);
    let mut count: u64 = 0;
    let mut grid: Vec<Vec<char>> = vec![];
    for line_result in reader.lines() {
        let line = line_result?;
        let characters: Vec<char> = line.chars().collect();
        grid.push(characters);
    }

    count += part_one(grid.clone());
    println!("Total rolls in part 1: {}", count);

    println!("Total rolls in part 2: {}", part_two(grid));

    Ok(())
}
