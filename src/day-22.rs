use std::{
    cmp::{max, min},
    collections::{HashMap, HashSet},
    fs::read_to_string,
};

const SIZE: usize = 50;

fn main() {
    let input = read_to_string("inputs/day-22.txt").expect("missing input file");
    let mut input = input.split("\n\n");
    let map = parse::map(input.next().expect("need map"));
    let actions = parse::input(input.next().expect("need actions"));
    let position = take_actions(SimpleMap::new(map.clone()), &actions);
    let score = calculate_score(position);
    println!("Part 1: {}", score);
    let position = take_actions(CubicMap::new(SIZE, map), &actions);
    print!("ended up in {position:?}");
    let score = calculate_score(position);
    println!("Part 2: {}", score);
}

#[derive(Clone, Copy, Debug)]
enum FaceDirection {
    Up,
    Down,
    Left,
    Right,
}

impl FaceDirection {
    fn turn(&self, direction: TurnDirection) -> Self {
        match (self, direction) {
            (Self::Up, TurnDirection::Left) => Self::Left,
            (Self::Up, TurnDirection::Right) => Self::Right,
            (Self::Left, TurnDirection::Left) => Self::Down,
            (Self::Left, TurnDirection::Right) => Self::Up,
            (Self::Right, TurnDirection::Left) => Self::Up,
            (Self::Right, TurnDirection::Right) => Self::Down,
            (Self::Down, TurnDirection::Left) => Self::Right,
            (Self::Down, TurnDirection::Right) => Self::Left,
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum TurnDirection {
    Left,
    Right,
}

impl TurnDirection {
    fn from(c: char) -> Self {
        match c {
            'L' => Self::Left,
            'R' => Self::Right,
            _ => panic!("unknown direction"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Action {
    Move(usize),
    Turn(TurnDirection),
}

type Actions = Vec<Action>;

type Point = (usize, usize);

type Position = (Point, FaceDirection);

#[derive(Debug, Clone)]
struct MapView {
    start: usize,
    end: usize,
    walls: HashSet<usize>,
}

impl Default for MapView {
    fn default() -> Self {
        Self {
            start: usize::MAX,
            end: 0,
            walls: Default::default(),
        }
    }
}

#[derive(Default, Debug, Clone)]
struct Map {
    h_view: HashMap<usize, MapView>,
    v_view: HashMap<usize, MapView>,
}

impl Map {
    fn starting_position(&self) -> Position {
        ((self.h_view[&0].start, 0), FaceDirection::Right)
    }

    fn is_free(&self, (x, y): Point) -> bool {
        !self.v_view[&x].walls.contains(&y)
    }
}

trait Navigate {
    fn do_move(&self, mut position: Position, steps: usize) -> Position {
        for _ in 0..steps {
            let (next_point, next_direction) = self.next_position(position);
            if self.is_free(next_point) {
                position = (next_point, next_direction);
            } else {
                return position;
            }
        }
        position
    }

    fn next_position(&self, position: Position) -> Position;
    fn is_free(&self, point: Point) -> bool;
    fn starting_position(&self) -> Position;
}

struct SimpleMap {
    map: Map,
}

impl SimpleMap {
    fn new(map: Map) -> Self {
        Self { map }
    }
}

impl Navigate for SimpleMap {
    fn next_position(&self, ((x, y), direction): Position) -> Position {
        let next_point = match direction {
            FaceDirection::Up => {
                let view = &self.map.v_view[&x];
                let y = if view.start == y { view.end } else { y - 1 };
                (x, y)
            }
            FaceDirection::Down => {
                let view = &self.map.v_view[&x];
                let y = if view.end == y { view.start } else { y + 1 };
                (x, y)
            }
            FaceDirection::Left => {
                let view = &self.map.h_view[&y];
                let x = if view.start == x { view.end } else { x - 1 };
                (x, y)
            }
            FaceDirection::Right => {
                let view = &self.map.h_view[&y];
                let x = if view.end == x { view.start } else { x + 1 };
                (x, y)
            }
        };
        (next_point, direction)
    }

    fn is_free(&self, point: Point) -> bool {
        self.map.is_free(point)
    }

    fn starting_position(&self) -> Position {
        self.map.starting_position()
    }
}

struct CubicMap {
    map: Map,
    size: usize,
}

impl CubicMap {
    fn new(size: usize, map: Map) -> Self {
        Self { size, map }
    }
}

impl Navigate for CubicMap {
    fn next_position(&self, ((x, y), direction): Position) -> Position {
        match direction {
            FaceDirection::Up => {
                let view = &self.map.v_view[&x];
                if view.start == y {
                    let rel = x % self.size;
                    match x / self.size {
                        0 => ((self.size, self.size + rel), FaceDirection::Right),
                        1 => ((0, self.size * 3 + rel), FaceDirection::Right),
                        2 => ((rel, self.size * 4 - 1), FaceDirection::Up),
                        _ => panic!("out of bounds"),
                    }
                } else {
                    ((x, y - 1), direction)
                }
            }
            FaceDirection::Down => {
                let view = &self.map.v_view[&x];
                if view.end == y {
                    let rel = x % self.size;
                    match x / self.size {
                        0 => ((self.size * 2 + rel, 0), FaceDirection::Down),
                        1 => ((self.size - 1, self.size * 3 + rel), FaceDirection::Left),
                        2 => ((self.size * 2 - 1, self.size + rel), FaceDirection::Left),
                        _ => panic!("out of bounds"),
                    }
                } else {
                    ((x, y + 1), direction)
                }
            }
            FaceDirection::Left => {
                let view = &self.map.h_view[&y];
                if view.start == x {
                    let rel = y % self.size;
                    match y / self.size {
                        0 => ((0, self.size * 3 - rel - 1), FaceDirection::Right),
                        1 => ((rel, self.size * 2), FaceDirection::Down),
                        2 => ((self.size, self.size - rel - 1), FaceDirection::Right),
                        3 => ((self.size + rel, 0), FaceDirection::Down),
                        _ => panic!("out of bounds"),
                    }
                } else {
                    ((x - 1, y), direction)
                }
            }
            FaceDirection::Right => {
                let view = &self.map.h_view[&y];
                if view.end == x {
                    let rel = y % self.size;
                    match y / self.size {
                        0 => (
                            (self.size * 2 - 1, self.size * 3 - rel - 1),
                            FaceDirection::Left,
                        ),
                        1 => ((self.size * 2 + rel, self.size - 1), FaceDirection::Up),
                        2 => (
                            (self.size * 3 - 1, self.size - rel - 1),
                            FaceDirection::Left,
                        ),
                        3 => ((self.size + rel, self.size * 3 - 1), FaceDirection::Up),
                        _ => panic!("out of bounds"),
                    }
                } else {
                    ((x + 1, y), direction)
                }
            }
        }
    }

    fn is_free(&self, point: Point) -> bool {
        self.map.is_free(point)
    }

    fn starting_position(&self) -> Position {
        self.map.starting_position()
    }
}

impl FromIterator<(Point, char)> for Map {
    fn from_iter<T: IntoIterator<Item = (Point, char)>>(points: T) -> Self {
        let mut map = Map::default();
        for ((x, y), c) in points {
            let mut h_view = map.h_view.entry(y).or_default();
            h_view.start = min(x, h_view.start);
            h_view.end = max(x, h_view.end);
            if c == '#' {
                h_view.walls.insert(x);
            }
            let mut v_view = map.v_view.entry(x).or_default();
            v_view.start = min(y, v_view.start);
            v_view.end = max(y, v_view.end);
            if c == '#' {
                v_view.walls.insert(y);
            }
        }
        map
    }
}

fn take_actions(map: impl Navigate, actions: &Actions) -> Position {
    let (mut current_point, mut current_direction) = map.starting_position();
    for action in actions {
        println!("at {current_point:?} facing {current_direction:?}: {action:?}");
        (current_point, current_direction) = match action {
            Action::Turn(turn_direction) => {
                (current_point, current_direction.turn(*turn_direction))
            }
            Action::Move(steps) => map.do_move((current_point, current_direction), *steps),
        }
    }
    (current_point, current_direction)
}

fn calculate_score(((x, y), facing_direction): Position) -> usize {
    (y + 1) * 1000
        + (x + 1) * 4
        + match facing_direction {
            FaceDirection::Right => 0,
            FaceDirection::Down => 1,
            FaceDirection::Left => 2,
            FaceDirection::Up => 3,
        }
}

mod parse {

    use super::{Action, Actions, Map, TurnDirection};

    use nom::{
        branch::alt,
        character::complete::{one_of, u32},
        multi::many1,
        IResult as NomResult,
    };

    pub(crate) fn input(input: &str) -> Actions {
        actions(input).expect("should be parseable").1
    }

    fn actions(input: &str) -> NomResult<&str, Actions> {
        many1(action)(input)
    }

    fn action(input: &str) -> NomResult<&str, Action> {
        alt((movement, turn))(input)
    }

    fn movement(input: &str) -> NomResult<&str, Action> {
        u32(input).map(|(input, num)| (input, Action::Move(num as usize)))
    }

    fn turn(input: &str) -> NomResult<&str, Action> {
        one_of("LR")(input)
            .map(|(input, op_str)| (input, Action::Turn(TurnDirection::from(op_str))))
    }

    pub(crate) fn map(input: &str) -> Map {
        input
            .split('\n')
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .filter(|(_, c)| *c != ' ')
                    .map(move |(x, c)| ((x, y), c))
            })
            .collect()
    }
}
