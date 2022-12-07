use std::{
    fs::File,
    io::{BufRead, BufReader},
};

const DIR_SIZE_LIMIT: usize = 100_000;
const TOTAL_SIZE: usize = 70_000_000;
const SPACE_NEEDED: usize = 30_000_000;

fn main() {
    let file = File::open("inputs/day-07.txt").expect("missing input file");
    let mut lines = BufReader::new(file)
        .lines()
        .map(|res| res.expect("should not fail reading line"));
    let entries = match Entry::from_iter(&mut lines) {
        Entry::Dir(entries) => entries,
        Entry::File(_) => panic!("only found one file!"),
    };
    let (disk_used, dir_sizes) = dir_sizes(&entries);
    let need_to_free = SPACE_NEEDED - (TOTAL_SIZE - disk_used);
    println!(
        "Part 1: {}",
        dir_sizes
            .iter()
            .filter(|size| **size <= DIR_SIZE_LIMIT)
            .sum::<usize>()
    );
    println!("Part 2: {}", dir_sizes
        .iter()
        .filter(|size| **size >= need_to_free)
        .min().expect("there should be a result")
    )
}

#[derive(Debug)]
enum Entry {
    Dir(Vec<Entry>),
    File(usize),
}

impl Entry {
    fn from_iter(feed: &mut impl Iterator<Item = String>) -> Self {
        let mut entries = vec![];
        while let Some(line) = feed.next() {
            match &line.split(' ').collect::<Vec<_>>()[..] {
                ["$", "cd", ".."] => break,
                ["$", "cd", _name] => entries.push(Entry::from_iter(feed)),
                ["$", "ls"] => {}    // skip for now
                ["dir", _name] => {} // skip for now
                [raw_size, _name] => entries.push(Entry::File(
                    raw_size.parse::<usize>().expect("should be a size"),
                )),
                e => panic!("unexpected entry {e:?}"),
            }
        }
        Entry::Dir(entries)
    }
}

fn dir_sizes(entries: &[Entry]) -> (usize, Vec<usize>) {
    let mut sizes = vec![];
    let mut current_size = 0;
    for entry in entries {
        match entry {
            Entry::File(size) => current_size += size,
            Entry::Dir(entries) => {
                let (sub_size, breakdown) = dir_sizes(entries);
                current_size += sub_size;
                sizes.extend(breakdown)
            }
        }
    }
    sizes.push(current_size);
    (current_size, sizes)
}
