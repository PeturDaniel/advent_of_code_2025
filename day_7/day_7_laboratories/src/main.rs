use std::{
    collections::{HashMap, HashSet},
    fs,
};

fn part_one(input: &String) -> u64 {
    let mut total_splits: u64 = 0;
    let mut beam_set: HashSet<usize> = HashSet::new();

    'outer: for line in input.lines() {
        // println!("Current line: {}", line);
        // println!("Current beam_set: {:#?}", beam_set);
        for index in 0..line.len() {
            if line.as_bytes()[index] as char == 'S' {
                beam_set.insert(index);
                continue 'outer;
            }
            if line.as_bytes()[index] as char == '^' && beam_set.contains(&index) {
                beam_set.insert(index - 1);
                beam_set.insert(index + 1);
                beam_set.remove(&index);
                total_splits += 1;
            }
        }
    }

    total_splits
}

fn part_two(input: &String) -> usize {
    let mut beam_map: HashMap<usize, usize> = HashMap::new();

    'outer: for line in input.lines() {
        // println!("Current line: {}", line);
        // println!("Current beam_set: {:#?}", beam_set);
        for index in 0..line.len() {
            if line.as_bytes()[index] as char == 'S' {
                beam_map.insert(index, 1);
                continue 'outer;
            }
            if line.as_bytes()[index] as char == '^' && beam_map.contains_key(&index) {
                let curr_count = beam_map[&index];
                match beam_map.get_mut(&(index - 1)) {
                    Some(count) => {
                        *count += curr_count;
                    }
                    None => {
                        beam_map.insert(index - 1, curr_count);
                    }
                }
                match beam_map.get_mut(&(index + 1)) {
                    Some(count) => {
                        *count += curr_count;
                    }
                    None => {
                        beam_map.insert(index + 1, curr_count);
                    }
                }
                beam_map.remove(&index);
            }
        }
        println!("Map: {:#?}", beam_map);
    }

    let mut total_timelines = 0;
    for pair in beam_map {
        total_timelines += pair.1;
    }

    total_timelines
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = fs::read_to_string("./input.txt")?;

    println!("Part one total split: {}", part_one(&input));
    println!("Part two total split: {}", part_two(&input));

    Ok(())
}
