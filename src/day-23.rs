use std::{
    cmp::{max, min},
    collections::{HashMap, HashSet},
    fs::read_to_string,
    ops::Add,
};

fn main() {
    let input = read_to_string("inputs/day-23.txt").expect("missing input file");
    let elves = parse_input(input.trim());
    let (elves_p1, _) = move_out(elves.clone(), Some(10));
    println!("Part 1: {}", empty_spaces(elves_p1));
    let (_, rounds) = move_out(elves, None);
    println!("Part 2: {}", rounds);
}

type Point = (i32, i32);
type Elves = HashSet<Point>;

#[derive(Clone, Copy)]
enum Direction {
    N,
    NE,
    E,
    SE,
    S,
    SW,
    W,
    NW,
}

impl Direction {
    fn offset(&self) -> Point {
        match self {
            Self::N => (0, -1),
            Self::NE => (1, -1),
            Self::E => (1, 0),
            Self::SE => (1, 1),
            Self::S => (0, 1),
            Self::SW => (-1, 1),
            Self::W => (-1, 0),
            Self::NW => (-1, -1),
        }
    }
}

impl Add<Direction> for Point {
    type Output = Point;

    fn add(self, rhs: Direction) -> Self::Output {
        let (x, y) = self;
        let (dx, dy) = rhs.offset();
        (x + dx, y + dy)
    }
}

struct PositionChecking {
    checks: [[Direction; 3]; 4],
    offset: usize,
}

impl PositionChecking {
    fn new() -> Self {
        Self {
            checks: [
                [Direction::N, Direction::NE, Direction::NW],
                [Direction::S, Direction::SE, Direction::SW],
                [Direction::W, Direction::NW, Direction::SW],
                [Direction::E, Direction::NE, Direction::SE],
            ],
            offset: 0,
        }
    }
}

impl Iterator for PositionChecking {
    type Item = Vec<[Direction; 3]>;

    fn next(&mut self) -> Option<Self::Item> {
        let next = [&self.checks[self.offset..4], &self.checks[..self.offset]].concat();
        self.offset = (self.offset + 1) % 4;
        Some(next)
    }
}

fn move_out(mut elves: Elves, max_rounds: Option<usize>) -> (Elves, usize) {
    let mut position_checking = PositionChecking::new();
    let mut round = 1;
    loop {
        let checks = position_checking.next().expect("always check");
        let mut suggestions: HashMap<Point, Elves> = HashMap::new();
        for elf in elves.iter().cloned() {
            suggestions.entry(elf).or_default().insert(elf);
            if neighbors(elf).iter().any(|n| elves.contains(n)) {
                'check: for [c1, c2, c3] in checks.iter() {
                    if [c1, c2, c3].iter().all(|c| !elves.contains(&(elf + **c))) {
                        suggestions.entry(elf).or_default().remove(&elf);
                        suggestions.entry(elf + *c1).or_default().insert(elf);
                        break 'check;
                    }
                }
            }
        }
        let new_elves = suggestions
            .into_iter()
            .flat_map(|(pos, elves)| match elves.len() {
                0 => vec![],
                1 => vec![pos],
                _ => elves.into_iter().collect(),
            })
            .collect();

        if elves == new_elves || Some(round - 1) == max_rounds {
            break;
        }
        round += 1;
        elves = new_elves;
    }
    (elves, round)
}

fn neighbors(point: Point) -> [Point; 8] {
    [
        point + Direction::N,
        point + Direction::NE,
        point + Direction::E,
        point + Direction::SE,
        point + Direction::S,
        point + Direction::SW,
        point + Direction::W,
        point + Direction::NW,
    ]
}

fn empty_spaces(elves: Elves) -> i32 {
    let (mut min_x, mut min_y, mut max_x, mut max_y) = (i32::MAX, i32::MAX, i32::MIN, i32::MIN);
    for (x, y) in elves.iter() {
        min_x = min(*x, min_x);
        min_y = min(*y, min_y);
        max_x = max(*x, max_x);
        max_y = max(*y, max_y);
    }

    ((max_x - min_x).abs() + 1) * ((max_y - min_y).abs() + 1) - elves.len() as i32
}

#[allow(unused)]
fn print_positions(elves: &Elves) {
    let (mut min_x, mut min_y, mut max_x, mut max_y) = (i32::MAX, i32::MAX, i32::MIN, i32::MIN);
    for (x, y) in elves {
        min_x = min(*x, min_x);
        min_y = min(*y, min_y);
        max_x = max(*x, max_x);
        max_y = max(*y, max_y);
    }

    let mut grid = vec![
        vec!['.'; (max_x - min_x).unsigned_abs() as usize + 1];
        (max_y - min_y).unsigned_abs() as usize + 1
    ];
    for (x, y) in elves {
        grid[(y - min_y) as usize][(x - min_x) as usize] = '#';
    }
    for line in grid.iter().map(|row| row.iter().collect::<String>()) {
        println!("{line}");
    }
    println!("\n\n");
}

fn parse_input(input: &str) -> HashSet<Point> {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().filter_map(move |(x, c)| {
                if c == '#' {
                    Some((x as i32, y as i32))
                } else {
                    None
                }
            })
        })
        .collect()
}
