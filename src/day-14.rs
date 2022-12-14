use std::{
    collections::{BTreeSet, HashMap},
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let file = File::open("inputs/day-14.txt").expect("missing input file");
    let mut cave = parse_initial_state(
        BufReader::new(file)
            .lines()
            .map(|l| l.expect("should be readable")),
    );
    let mut poured_sand = cave.pour_sand_until_stable();
    println!("Part 1: {}", poured_sand);
    cave.floor = cave
        .blocked
        .values()
        .map(|v| v.iter().rev().next().expect("for sure has a point"))
        .max()
        .map(|m| m + 2);
    poured_sand += cave.pour_sand_until_stable();
    println!("Part 2: {}", poured_sand);
}

type Point = (i32, i32);

struct Cave {
    blocked: HashMap<i32, BTreeSet<i32>>,
    floor: Option<i32>,
}

impl Cave {
    fn new() -> Self {
        Self {
            blocked: HashMap::new(),
            floor: None,
        }
    }

    fn blocker_below(&self, (x, y): Point) -> Option<Point> {
        self.blocked
            .get(&x)
            .and_then(|s| s.range(y..).next().map(|y| (x, *y)))
            .or_else(|| self.floor.map(|floor_y| (x, floor_y)))
    }

    fn is_blocked(&self, (x, y): Point) -> bool {
        match self.floor {
            Some(floor_y) if floor_y <= y => true,
            _ => match self.blocked.get(&x) {
                Some(s) => s.contains(&y),
                None => false,
            },
        }
    }

    fn insert(&mut self, (x, y): Point) {
        self.blocked.entry(x).or_default().insert(y);
    }

    fn pour_sand_until_stable(&mut self) -> usize {
        let sand_entry_point = (500, 0);
        let mut poured_sand = 0;
        while let Some(point) = self.add_sand(sand_entry_point) {
            poured_sand += 1;
            if point == sand_entry_point {
                break;
            }
        }
        poured_sand
    }

    fn add_sand(&mut self, mut sand_point: Point) -> Option<Point> {
        loop {
            sand_point = match self.blocker_below(sand_point) {
                None => return None,
                Some((xb, yb)) => {
                    let left_blocker = (xb - 1, yb);
                    let right_blocker = (xb + 1, yb);
                    if !self.is_blocked(left_blocker) {
                        left_blocker
                    } else if !self.is_blocked(right_blocker) {
                        right_blocker
                    } else {
                        self.insert((xb, yb - 1));
                        return Some((xb, yb - 1));
                    }
                }
            }
        }
    }
}

fn parse_initial_state(lines: impl Iterator<Item = String>) -> Cave {
    let all_points = lines.flat_map(|line| {
        line.split(" -> ")
            .fold(vec![], |mut line, raw_point| {
                let mut raw_point = raw_point.split(',');
                let point = (
                    raw_point
                        .next()
                        .expect("should have next")
                        .parse::<i32>()
                        .expect("should be a number"),
                    raw_point
                        .next()
                        .expect("should have next")
                        .parse::<i32>()
                        .expect("should be a number"),
                );
                match line.last() {
                    Some(last_point) => line.extend(points_between(*last_point, point)),
                    None => line.push(point),
                }
                line
            })
            .into_iter()
    });
    all_points
        .into_iter()
        .fold(Cave::new(), |mut cave, point| {
            cave.insert(point);
            cave
        })
}

fn points_between((xa, ya): Point, (xb, yb): Point) -> PointLine {
    let (dx, dy) = (num::signum(xb - xa), num::signum(yb - ya));
    PointLine {
        from: (xa, ya),
        to: (xb, yb),
        offset: (dx, dy),
    }
}

struct PointLine {
    from: Point,
    to: Point,
    offset: Point,
}

impl Iterator for PointLine {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        if self.from == self.to {
            None
        } else {
            let (x, y) = self.from;
            let (dx, dy) = self.offset;
            let next = (x + dx, y + dy);
            self.from = next;
            Some(next)
        }
    }
}
