use std::{
    cmp::Ordering,
    fs::File,
    io::{BufRead, BufReader},
};

const DECRYPTION_KEY: i64 = 811589153;

fn main() {
    let file = File::open("inputs/day-20.txt").expect("missing input file");
    let encrypted_file = BufReader::new(file)
        .lines()
        .map(|s| s.expect("must read").parse::<i64>().expect("must be num"))
        .collect::<Vec<_>>();

    let rotated_file = rotate_file(&encrypted_file, 1);
    let coordinate = get_coords(&rotated_file);
    println!("Part 1: {}", coordinate);

    let file_with_key = encrypted_file
        .iter()
        .cloned()
        .map(|v| v * DECRYPTION_KEY)
        .collect::<Vec<_>>();
    let rotated_file = rotate_file(&file_with_key, 10);
    let coordinate = get_coords(&rotated_file);
    println!("Part 2: {}", coordinate);
}

fn get_coords(input: &[i64]) -> i64 {
    let (zero_index, _) = input
        .iter()
        .enumerate()
        .find(|(_, val)| **val == 0)
        .expect("zero is in there");
    let len = input.len();
    input[(zero_index + 1000) % len]
        + input[(zero_index + 2000) % len]
        + input[(zero_index + 3000) % len]
}

fn rotate_file(initial: &[i64], times: usize) -> Vec<i64> {
    let len = initial.len();
    let mut rotated = initial.iter().cloned().enumerate().collect::<Vec<_>>();

    for _ in 0..times {
        for i in 0..len {
            let (index, (orig, val)) = rotated
                .iter()
                .enumerate()
                .find(|(_, (orig, _))| *orig == i)
                .expect("for sure we have a value");

            let raw_index = index as i64 + val;
            let mut new_index = raw_index % (len as i64 - 1);
            if new_index <= 0 {
                new_index += len as i64 - 1;
            }
            let new_index = new_index as usize;
            rotated = match new_index.cmp(&index) {
                Ordering::Less => {
                    let mut new_rotated = Vec::with_capacity(len);
                    new_rotated.extend_from_slice(&rotated[..new_index]);
                    new_rotated.push((*orig, *val));
                    new_rotated.extend_from_slice(&rotated[new_index..index]);
                    new_rotated.extend_from_slice(&rotated[index + 1..len]);
                    new_rotated
                }
                Ordering::Greater => {
                    let mut new_rotated = Vec::with_capacity(len);
                    new_rotated.extend_from_slice(&rotated[..index]);
                    new_rotated.extend_from_slice(&rotated[index + 1..=new_index]);
                    new_rotated.push((*orig, *val));
                    new_rotated.extend_from_slice(&rotated[new_index + 1..len]);
                    new_rotated
                }
                _ => rotated,
            };
        }
    }
    rotated.into_iter().map(|(_, v)| v).collect()
}
