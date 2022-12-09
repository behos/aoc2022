use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let file = File::open("inputs/day-09.txt").expect("missing input file");
    let lines = BufReader::new(file)
        .lines()
        .map(|res| res.expect("should not fail reading line"));
    let mut trail = get_head_trail(lines);
    trail = get_follower_trail(trail);
    println!("Part 1: {}", trail.iter().collect::<HashSet<_>>().len());
    for _ in 0..8 {
        trail = get_follower_trail(trail);
    }
    println!("Part 2: {}", trail.iter().collect::<HashSet<_>>().len());
}

fn get_head_trail(directions: impl Iterator<Item = String>) -> Vec<(i32, i32)> {
    let mut trail = vec![(0, 0)];
    for direction in directions {
        let mut parts = direction.split(' ');
        let dir = parts.next().expect("should have a part");
        let amount = parts
            .next()
            .expect("should have a second part")
            .parse::<i32>()
            .expect("should be a number");
        let (dx, dy) = offset_for_direction(dir);
        for _ in 0..amount {
            let (lx, ly) = trail.last().expect("should have a position");
            trail.push((lx + dx, ly + dy));
        }
    }
    trail
}

fn get_follower_trail(head: Vec<(i32, i32)>) -> Vec<(i32, i32)> {
    head.into_iter().fold(vec![], |mut trail, (hx, hy)| {
        let (tx, ty) = trail.last().unwrap_or(&(0, 0));
        let (dx, dy) = (hx - tx, hy - ty);
        let (mx, my) = if dx.abs() > 1 || dy.abs() > 1 {
            (num::signum(dx), num::signum(dy))
        } else {
            (0, 0)
        };
        trail.push((tx + mx, ty + my));
        trail
    })
}

fn offset_for_direction(direction: &str) -> (i32, i32) {
    match direction {
        "U" => (0, 1),
        "D" => (0, -1),
        "L" => (-1, 0),
        "R" => (1, 0),
        err => panic!("what sort of direction is {err}"),
    }
}
