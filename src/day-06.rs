use std::{
    collections::{hash_map::Entry, HashMap, VecDeque},
    fs::read_to_string,
};

const MARKER_SIZE: usize = 4;
const PACKET_SIZE: usize = 14;

fn main() {
    let input = read_to_string("inputs/day-06.txt").expect("missing input file");
    println!("Part 1: {}", find_marker(&input, MARKER_SIZE));
    println!("Part 2: {}", find_marker(&input, PACKET_SIZE));
}

fn find_marker(input: &str, size: usize) -> usize {
    let mut buffer = VecDeque::with_capacity(size + 1);
    let mut index = HashMap::new();
    for (i, c) in input.chars().enumerate() {
        buffer.push_back(c);
        index.entry(c).and_modify(|e| *e += 1).or_insert(1);
        if buffer.len() > size {
            let to_remove = buffer.pop_front().expect("should have elements");
            match index.entry(to_remove).and_modify(|e| *e -= 1) {
                Entry::Occupied(e) if *e.get() == 0 => {
                    e.remove();
                }
                _ => (),
            }
        }
        if index.keys().len() == size {
            return i + 1;
        }
    }
    panic!("no marker found")
}
