use std::fs::read_to_string;

fn main() {
    let input = read_to_string("inputs/day-08.txt").expect("missing input file");
    let trees = to_trees(parse_input(input));
    println!("Part 1: {}", count_visible(&trees));
    println!("Part 2: {}", max_scenic_score(&trees));
}

const OFFSETS: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, 1), (0, -1)];

#[derive(Debug)]
struct Tree {
    // directions in order NSEW. Shows if we are blocked, and how far ahead.
    blockers: [(bool, usize); 4],
}

type Grid = Vec<Vec<u32>>;

impl Tree {
    fn from(grid: &Grid, x: usize, y: usize) -> Self {
        let mut blockers = [(false, 0); 4];
        for (i, offset) in OFFSETS.iter().enumerate() {
            blockers[i] = find_blocker(grid, x, y, offset);
        }
        Self { blockers }
    }

    fn is_visible(&self) -> bool {
        self.blockers.iter().any(|b| !b.0)
    }

    fn scenic_score(&self) -> usize {
        self.blockers.iter().map(|b| b.1).product()
    }
}

fn parse_input(input: String) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|r| {
            r.chars()
                .map(|h| h.to_digit(10).expect("should be int"))
                .collect()
        })
        .collect()
}

fn to_trees(grid: Vec<Vec<u32>>) -> Vec<Tree> {
    (0..grid.len())
        .flat_map(|x| {
            (0..grid[x].len())
                .map(|y| Tree::from(&grid, x, y))
                .collect::<Vec<_>>()
        })
        .collect()
}

fn find_blocker(grid: &Grid, x: usize, y: usize, (dx, dy): &(i32, i32)) -> (bool, usize) {
    let (mut nx, mut ny) = (x as i32, y as i32);
    let height = grid[x][y];
    let mut distance = 0;
    loop {
        nx += dx;
        ny += dy;
        if is_out_of_bounds(grid, nx, ny) {
            return (false, distance);
        } else if grid[nx as usize][ny as usize] >= height {
            return (true, distance + 1);
        }
        distance += 1;
    }
}

fn is_out_of_bounds(grid: &Grid, x: i32, y: i32) -> bool {
    x < 0 || x == grid.len() as i32 || y < 0 || y == grid[x as usize].len() as i32
}

fn count_visible(trees: &[Tree]) -> usize {
    trees.iter().filter(|t| t.is_visible()).count()
}

fn max_scenic_score(trees: &[Tree]) -> usize {
    trees
        .iter()
        .map(|t| t.scenic_score())
        .max()
        .expect("there should be a max")
}
