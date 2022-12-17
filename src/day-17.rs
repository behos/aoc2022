use std::{collections::{BTreeSet, hash_map::DefaultHasher, HashMap}, fs::read_to_string, hash::Hasher};

const P1_TARGET: usize = 2022;
const P2_TARGET: usize = 1_000_000_000_000;

fn main() {
    let input = read_to_string("inputs/day-17.txt").expect("missing input file");
    let input = input.trim();
    let height = play_rock_stacking(input.to_string(), P1_TARGET);
    println!("Part 1: {}", height);
    let height = play_rock_stacking(input.to_string(), P2_TARGET);
    println!("Part 2: {}", height);
}

type Point = (i32, i32);
type Shape = Vec<Point>;

struct Board {
    rocks: Vec<BTreeSet<usize>>,
    current_shape: Option<Shape>,
}

impl Board {
    fn new() -> Self {
        Self {
            rocks: vec![BTreeSet::new(); 7],
            current_shape: None,
        }
    }

    fn move_shape(&mut self, (dx, dy): Point) {
        self.current_shape = self.current_shape.take().map(|mut shape| {
            for point in &mut shape {
                let (x, y) = point;
                *point = (*x + dx, *y + dy)
            }
            shape
        });
    }

    fn insert_shape(&mut self, shape: Shape) {
        self.current_shape = Some(shape);
        self.move_shape((2, 0)); // 2 units from the right.
        self.move_shape((0, self.height() as i32 + 3)); // 3 units above the highest rock.
    }

    fn do_move(&mut self, op: char) {
        let offset = match op {
            '<' => (-1, 0),
            '>' => (1, 0),
            _ => panic!("unpredictable jet flow {op}"),
        };

        if self.can_move(offset) {
            self.move_shape(offset);
        }

        if !self.can_move((0, -1)) {
            self.solidify();
        } else {
            self.move_shape((0, -1))
        }
    }

    fn height(&self) -> usize {
        self.rocks
            .iter()
            .map(|s| s.last().map(|h| h + 1).unwrap_or(0))
            .max()
            .unwrap_or(0)
    }

    fn floor(&self) -> usize {
        self.rocks
            .iter()
            .map(|s| s.last().map(|h| h + 1).unwrap_or(0))
            .min()
            .unwrap_or(0)
    }

    fn can_move(&self, (dx, dy): Point) -> bool {
        self.current_shape
            .as_ref()
            .map(|shape| shape.iter().all(|(x, y)| self.is_empty((x + dx, y + dy))))
            .unwrap_or(false)
    }

    fn solidify(&mut self) {
        if let Some(shape) = self.current_shape.take() {
            for (x, y) in shape.into_iter() {
                self.rocks[x as usize].insert(y as usize);
            }
        }
    }

    fn is_empty(&self, (x, y): Point) -> bool {
        self.in_bounds((x, y)) && !self.rocks[x as usize].contains(&(y as usize))
    }

    fn in_bounds(&self, (x, y): Point) -> bool {
        (0..7).contains(&x) && y >= 0
    }

    #[allow(unused)]
    fn draw(&self) {
        println!("{:?}", self.rocks);
        let lines = (0..self.height() + 7).rev().map(|y| {
            self.rocks
                .iter()
                .enumerate()
                .map(|(x, col)| {
                    if col.contains(&y) {
                        '#'
                    } else {
                        match &self.current_shape {
                            Some(shape) if shape.contains(&(x as i32, y as i32)) => '@',
                            _ => '.',
                        }
                    }
                })
                .collect::<String>()
        });
        println!("\n\n\n");
        for line in lines {
            println!("{}", line)
        }
    }
}

fn play_rock_stacking(input: String, mut limit: usize) -> usize {
    let ordered_shapes = [
        vec![(0, 0), (1, 0), (2, 0), (3, 0)],         // Horizontal line
        vec![(0, 1), (1, 1), (2, 1), (1, 2), (1, 0)], // Cross
        vec![(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)], // L shape
        vec![(0, 0), (0, 1), (0, 2), (0, 3)],         // Vertical line
        vec![(0, 0), (0, 1), (1, 0), (1, 1)],         // Square
    ];
    let mut board = Board::new();
    let mut input = input.chars().enumerate().cycle();
    let shapes = ordered_shapes.iter().enumerate().cycle().enumerate();
    let mut heights = HashMap::new();
    let mut height_offset = 0;
    for (shape_count, (shape_idx, shape)) in shapes {
        if shape_count == limit {
            break;
        }

        // Play the round
        board.insert_shape(shape.to_vec());
        let mut move_idx = 0;
        while board.current_shape.is_some() {
            let (idx, ch) = input.next().expect("moves are forever");
            board.do_move(ch);
            move_idx = idx;
        }

        // Now let's hash and store the height to find any state repetitions
        let mut hasher = DefaultHasher::new();
        hasher.write_usize(move_idx);  // If we are in the same move cycle
        hasher.write_usize(shape_idx);  // and in the same shape cycle
        let floor = board.floor();
        for col in &board.rocks {
            hasher.write_usize(col.last().unwrap_or(&0) - floor);  // and the floor looks the same
        }
        let hash = hasher.finish();

        let height = board.height();

        // If there was a repetition of this state, let's replay as many times as possible
        if let Some((last_shape_count, last_height)) = heights.get(&hash) {
            let height_diff = height - last_height;
            let shapes_diff = shape_count - last_shape_count;
            while shape_count + shapes_diff < limit {
                limit -= shapes_diff;
                height_offset += height_diff;
            }
        } else {
            heights.insert(hash, (shape_count, height));
        }
    }

    board.height() + height_offset
}
