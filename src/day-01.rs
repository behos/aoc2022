use std::{collections::BinaryHeap, fs::read_to_string};

fn main() {
    let input = read_to_string("inputs/day-01.txt").expect("missing input file");
    let sorted: Vec<_> = heap_of_calories(&input).into_sorted_vec();
    println!("Part 1 - {}", sorted[sorted.len() - 1]);
    println!(
        "Part 2 - {}",
        sorted[sorted.len() - 3..].iter().sum::<usize>()
    );
}

fn heap_of_calories(input: &str) -> BinaryHeap<usize> {
    input
        .split('\n')
        .map(|l| l.parse::<usize>().ok())
        .fold(
            (BinaryHeap::new(), 0),
            |(mut heap, current), line| match line {
                None => {
                    heap.push(current);
                    (heap, 0)
                }
                Some(calorie) => (heap, current + calorie),
            },
        )
        .0
}
