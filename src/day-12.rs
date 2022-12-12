use std::{collections::HashMap, fs::read_to_string};

fn main() {
    let input = read_to_string("inputs/day-12.txt").expect("missing input file");
    let (start, end, grid) = parse_input(input);
    let starting_points = grid
        .iter()
        .filter(|(_, p)| p.elevation == 0)
        .map(|(c, _)| c);
    println!("Part 1: {}", shortest_path(start, end, &mut grid.clone()));
    println!(
        "Part 2: {}",
        starting_points
            .map(|s| shortest_path(*s, end, &mut grid.clone()))
            .min()
            .expect("for sure there is a min")
    );
}

#[derive(Clone, Copy)]
struct Point {
    elevation: u32,
    min_distance: usize,
}
type Coords = (i32, i32);
type Grid = HashMap<Coords, Point>;

fn parse_input(input: String) -> (Coords, Coords, Grid) {
    let mut grid = Grid::new();
    let mut start = (0, 0);
    let mut end = (0, 0);
    for (x, line) in input.lines().enumerate() {
        for (y, c) in line.chars().enumerate() {
            let x = x as i32;
            let y = y as i32;
            let elevation = match c {
                'S' => {
                    start = (x, y);
                    0
                }
                'E' => {
                    end = (x, y);
                    26
                }
                c => c as u32 - 'a' as u32,
            };
            grid.insert(
                (x, y),
                Point {
                    elevation,
                    min_distance: usize::MAX,
                },
            );
        }
    }
    (start, end, grid)
}

fn shortest_path(start: Coords, end: Coords, grid: &mut Grid) -> usize {
    let mut to_visit = vec![start];
    grid.entry(start).and_modify(|e| e.min_distance = 0);
    while let Some(point) = to_visit.pop() {
        let distance = grid[&point].min_distance + 1;
        for neighbor in get_neighbors(point, grid) {
            to_visit.push(neighbor);
            grid.entry(neighbor)
                .and_modify(|e| e.min_distance = distance);
        }
    }
    grid[&end].min_distance
}

fn get_neighbors((x, y): Coords, grid: &Grid) -> Vec<Coords> {
    let mut neighbors = vec![];
    let point = &grid[&(x, y)];
    for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
        let candidate = (x + dx, y + dy);
        if let Some(n_point) = grid.get(&candidate) {
            if n_point.elevation <= point.elevation + 1
                && n_point.min_distance > point.min_distance + 1
            {
                neighbors.push(candidate)
            }
        }
    }
    neighbors
}
