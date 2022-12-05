use std::fs::read_to_string;

fn main() {
    let input = read_to_string("inputs/day-05.txt").expect("missing input file");
    let mut parts = input.trim().split("\n\n");
    let mut stacks_v9000 =
        parse_initial_state(parts.next().expect("should have initial state section"));
    let mut stacks_v9001 = stacks_v9000.to_vec();
    let moves = parse_moves(parts.next().expect("should have list of moves"));
    apply_moves(&moves, &mut stacks_v9000, Version::V9000);
    apply_moves(&moves, &mut stacks_v9001, Version::V9001);
    println!("Part 1: {}", top_crates(&stacks_v9000));
    println!("Part 2: {}", top_crates(&stacks_v9001));
}

macro_rules! next_num {
    ($iter:ident) => {
        $iter
            .nth(1)
            .expect("should have amount")
            .parse::<usize>()
            .expect("should be num")
    };
}

fn parse_initial_state(raw_state: &str) -> Vec<Vec<char>> {
    let mut lines = raw_state.split('\n').rev();
    let num_stacks = lines
        .next()
        .expect("needed at least 1 line")
        .split("  ")
        .count();
    let mut stacks = vec![vec![]; num_stacks];
    for line in lines {
        let mut chars = line.chars();
        for (i, stack) in stacks.iter_mut().enumerate() {
            match chars.nth(if i == 0 { 1 } else { 3 }) {
                Some(c) if c != ' ' => stack.push(c),
                _ => (),
            }
        }
    }
    stacks
}

#[derive(PartialEq, Eq)]
enum Version {
    V9000,
    V9001,
}

type Move = (usize, usize, usize);

fn parse_moves(raw_moves: &str) -> Vec<Move> {
    raw_moves
        .split('\n')
        .map(|line| {
            let mut parts = line.split(' ');
            (next_num!(parts), next_num!(parts) - 1, next_num!(parts) - 1)
        })
        .collect()
}

fn apply_moves(moves: &Vec<Move>, stacks: &mut [Vec<char>], version: Version) {
    for (amount, from, to) in moves {
        let split_at = stacks[*from].len() - amount;
        let split_off = stacks[*from].split_off(split_at).into_iter();
        match version {
            Version::V9000 => stacks[*to].extend(split_off.rev()),
            Version::V9001 => stacks[*to].extend(split_off),
        }
    }
}

fn top_crates(stacks: &[Vec<char>]) -> String {
    stacks
        .iter()
        .map(|stack| stack.last().expect("should not be empty"))
        .collect()
}
