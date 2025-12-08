use std::{collections::HashSet, fs};

fn euclidean_distance(point_a: Vec<i64>, point_b: Vec<i64>) -> i64 {
    let x = (point_a[0] - point_b[0]).pow(2);
    let y = (point_a[1] - point_b[1]).pow(2);
    let z = (point_a[2] - point_b[2]).pow(2);
    // let distance = (x + y + z).isqrt();
    let distance = x + y + z;
    distance
}

fn part_one(distances: &Vec<(i64, i64, i64)>) -> i64 {
    let mut total: i64 = 1;

    let mut circuits: Vec<HashSet<i64>> = vec![];

    'outer: for i in 0..1000 {
        let mut index_a: i64 = -1;
        let mut index_b: i64 = -1;
        for (index, circuit) in circuits.iter().enumerate() {
            if circuit.contains(&distances[i].1) && circuit.contains(&distances[i].2) {
                continue 'outer;
            }
            if circuit.contains(&distances[i].1) {
                index_a = index as i64;
            }
            if circuit.contains(&distances[i].2) {
                index_b = index as i64;
            }
        }
        if index_a == -1 && index_b == -1 {
            let mut new_circuit = HashSet::new();
            new_circuit.insert(distances[i].1);
            new_circuit.insert(distances[i].2);
            circuits.push(new_circuit);
            continue 'outer;
        }
        if index_a != -1 && index_b != -1 {
            let lower;
            let higher;
            if index_a < index_b {
                lower = index_a;
                higher = index_b;
            } else {
                lower = index_b;
                higher = index_a;
            }
            let circuit_b = circuits.remove(higher as usize);
            circuits[lower as usize].extend(circuit_b.into_iter());
            continue 'outer;
        }
        if index_a != -1 {
            circuits[index_a as usize].insert(distances[i].2);
            continue 'outer;
        }
        if index_b != -1 {
            circuits[index_b as usize].insert(distances[i].1);
            continue 'outer;
        }
        // println!("Current circuits:");
    }

    let mut lengths: Vec<usize> = circuits.iter().map(|c| c.len()).collect();

    lengths.sort_unstable_by(|a, b| b.cmp(a));
    // println!("{:#?}", lengths);

    for index in 0..3 {
        total *= lengths[index] as i64;
    }

    total
}

fn part_two(distances: Vec<(i64, i64, i64)>, coordinates: Vec<Vec<i64>>) -> i64 {
    let mut circuits: Vec<HashSet<i64>> = vec![];

    let mut final_a = 0;
    let mut final_b = 0;

    'outer: for i in 0..distances.len() {
        let mut index_a: i64 = -1;
        let mut index_b: i64 = -1;
        for (index, circuit) in circuits.iter().enumerate() {
            if circuit.contains(&distances[i].1) && circuit.contains(&distances[i].2) {
                continue 'outer;
            }
            if circuit.contains(&distances[i].1) {
                index_a = index as i64;
            }
            if circuit.contains(&distances[i].2) {
                index_b = index as i64;
            }
        }
        if index_a == -1 && index_b == -1 {
            let mut new_circuit = HashSet::new();
            new_circuit.insert(distances[i].1);
            new_circuit.insert(distances[i].2);
            circuits.push(new_circuit);
            continue 'outer;
        }
        if index_a != -1 && index_b != -1 {
            let lower;
            let higher;
            if index_a < index_b {
                lower = index_a;
                higher = index_b;
            } else {
                lower = index_b;
                higher = index_a;
            }
            let circuit_b = circuits.remove(higher as usize);
            circuits[lower as usize].extend(circuit_b.into_iter());
            final_a = coordinates[distances[i].1 as usize][0];
            final_b = coordinates[distances[i].2 as usize][0];
            continue 'outer;
        }
        if index_a != -1 {
            circuits[index_a as usize].insert(distances[i].2);
            final_a = coordinates[distances[i].1 as usize][0];
            final_b = coordinates[distances[i].2 as usize][0];
            continue 'outer;
        }
        if index_b != -1 {
            circuits[index_b as usize].insert(distances[i].1);
            final_a = coordinates[distances[i].1 as usize][0];
            final_b = coordinates[distances[i].2 as usize][0];
            continue 'outer;
        }
        // println!("Current circuits:");
    }

    final_a * final_b
}

fn main() -> Result<(), Box<dyn ::std::error::Error>> {
    let input = fs::read_to_string("./input.txt")?;

    let mut coordinates: Vec<Vec<i64>> = vec![];
    for line in input.lines() {
        let lines = line.split(",");
        let mut point: Vec<i64> = vec![];
        for number in lines {
            point.push(number.parse().unwrap());
        }
        coordinates.push(point);
    }
    let mut distances: Vec<(i64, i64, i64)> = vec![];
    for point_index in 0..coordinates.len() - 1 {
        for second_point_index in point_index + 1..coordinates.len() {
            let point_a = coordinates[point_index].clone();
            let point_b = coordinates[second_point_index].clone();
            let distance = euclidean_distance(point_a, point_b);
            distances.push((distance, point_index as i64, second_point_index as i64));
        }
    }

    distances.sort();

    // println!("Part one: {}", part_one(&distances));

    println!("Part two: {}", part_two(distances, coordinates));

    Ok(())
}
