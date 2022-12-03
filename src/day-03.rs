use std::{collections::HashSet, fs::read_to_string};

fn main() {
    let input = read_to_string("inputs/day-03.txt").expect("missing input file");
    let rucksacks: Vec<_> = input.split('\n').filter(|s| &"" != s).collect();
    println!("Part 1: {}", priority_sum(&rucksacks));
    println!("Part 2: {}", badge_priority_sum(&rucksacks));
}

fn priority_sum(inputs: &[&str]) -> usize {
    inputs
        .iter()
        .map(|line| priority(rucksack_misplaced(line)))
        .sum()
}

fn rucksack_misplaced(line: &str) -> char {
    let chars = line.chars().collect::<Vec<_>>();
    let sack_a = HashSet::<&char>::from_iter(&chars[..chars.len() / 2]);
    let sack_b = HashSet::<&char>::from_iter(&chars[chars.len() / 2..]);
    **(sack_a
        .intersection(&sack_b)
        .next()
        .expect("no common items"))
}

fn priority(c: char) -> usize {
    c as usize
        - if c.is_lowercase() {
            'a' as usize - 1
        } else {
            'A' as usize - 27
        }
}

fn badge_priority_sum(inputs: &[&str]) -> usize {
    inputs
        .chunks(3)
        .map(|group| priority(group_badge(group)))
        .sum()
}

fn group_badge(group: &[&str]) -> char {
    group
        .iter()
        .fold(None, |common, rucksack| {
            let set = HashSet::<char>::from_iter(rucksack.chars());
            match common {
                None => Some(set),
                Some(common) => Some(HashSet::<char>::from_iter(
                    common.intersection(&set).cloned(),
                )),
            }
        })
        .expect("uninitialized fold")
        .into_iter()
        .next()
        .expect("no common items")
}
