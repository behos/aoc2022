use std::{
    collections::{hash_map::Entry, HashMap, VecDeque},
    fs::read_to_string,
    ops::Add,
};

fn main() {
    let input = read_to_string("inputs/day-24.txt").expect("missing input file");
    let valley = parse_input(input.trim());
    let dest = (valley.width - 1, valley.height - 1);
    let weather_navigation = gather_weather_navigation(valley);
    let min_steps = find_path(weather_navigation.clone(), (0, (0, 0)), dest);
    println!("Part 1: {}", min_steps);
    let round_trip = shortest_round_trip(weather_navigation, dest);
    println!("Part 2: {}", round_trip);
}

type Point = (isize, isize);
type Offset = (isize, isize);
type PointInTime = (usize, Point);
type WeatherNavigation = Vec<HashMap<Point, usize>>;

#[derive(Clone, Copy, Debug)]
enum Direction {
    N,
    E,
    S,
    W,
}

impl Direction {
    fn offset(&self) -> Offset {
        match self {
            Self::N => (0, -1),
            Self::E => (1, 0),
            Self::S => (0, 1),
            Self::W => (-1, 0),
        }
    }
}

impl Add<Direction> for Point {
    type Output = Offset;

    fn add(self, rhs: Direction) -> Self::Output {
        let (x, y) = self;
        let (dx, dy) = rhs.offset();
        ((x + dx), (y + dy))
    }
}

#[derive(Debug)]
struct Valley {
    width: isize,
    height: isize,
    blizzards: HashMap<Point, Vec<Direction>>,
}

impl Valley {
    fn wraparound(&self, (dx, dy): Offset) -> Point {
        (
            if dx == -1 {
                self.width - 1
            } else {
                dx % self.width
            },
            if dy == -1 {
                self.height - 1
            } else {
                dy % self.height
            },
        )
    }
}

fn gather_weather_navigation(mut valley: Valley) -> WeatherNavigation {
    let mut spaces_in_time = vec![];
    loop {
        let blizzards = &valley.blizzards;
        let empty_spaces: HashMap<_, _> = (0..valley.width)
            .flat_map(|x| {
                (0..valley.height)
                    .map(move |y| (x, y))
                    .filter_map(|(x, y)| {
                        if blizzards.contains_key(&(x, y)) {
                            None
                        } else {
                            Some(((x, y), usize::MAX))
                        }
                    })
            })
            .collect();
        if !spaces_in_time.is_empty() && spaces_in_time[0] == empty_spaces {
            // found cycle point
            break;
        }
        spaces_in_time.push(empty_spaces);
        valley = move_blizzards(valley);
    }
    spaces_in_time
}

fn move_blizzards(valley: Valley) -> Valley {
    let mut blizzards: HashMap<Point, Vec<Direction>> = HashMap::new();
    for (point, directions) in &valley.blizzards {
        for direction in directions {
            blizzards
                .entry(valley.wraparound(*point + *direction))
                .or_default()
                .push(*direction)
        }
    }
    Valley {
        blizzards,
        ..valley
    }
}

fn shortest_round_trip(nav: WeatherNavigation, to: Point) -> usize {
    let trip = find_path(nav.clone(), (0, (0, 0)), to);
    let trip = find_path(nav.clone(), (trip + 1, to), (0, 0));
    find_path(nav, (trip + 1, (0, 0)), to)
}

fn find_path(mut nav: WeatherNavigation, from: PointInTime, to: Point) -> usize {
    explore_weather(&mut nav, from);
    nav.iter()
        .filter_map(|spaces| spaces.get(&to))
        .min()
        .expect("can never reach destination")
        + 2
}

fn explore_weather(nav: &mut WeatherNavigation, (time, from): PointInTime) {
    let cycle = nav.len();
    let mut points_to_explore: VecDeque<_> = (time..time + nav.len()).into_iter()
        .filter_map(|time| {
            if nav[time % cycle].contains_key(&from) {
                Some((time, from))
            } else {
                None
            }
        })
        .collect();

    // init starting points in map
    for (time, point) in points_to_explore.iter() {
        nav[*time % cycle].entry(*point).and_modify(|e| *e = *time);
    }

    while let Some((time, point)) = points_to_explore.pop_back() {
        for (ntime, npoint) in neighbors((time, point)) {
            let rel_time = ntime % nav.len();
            if let Entry::Occupied(mut e) = nav[rel_time].entry(npoint) {
                if e.get() > &time {
                    e.insert(time);
                    points_to_explore.push_front((ntime, npoint));
                }
            }
        }
    }
}

fn neighbors((time, point): PointInTime) -> Vec<PointInTime> {
    let mut temporal_neighbors = vec![(time + 1, point)];
    temporal_neighbors.extend(
        [Direction::N, Direction::E, Direction::S, Direction::W]
            .into_iter()
            .map(move |d| (time + 1, point + d)),
    );
    temporal_neighbors
}

fn parse_input(input: &str) -> Valley {
    let blizzards = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().filter_map(move |(x, c)| {
                match c {
                    '^' => Some(Direction::N),
                    'v' => Some(Direction::S),
                    '>' => Some(Direction::E),
                    '<' => Some(Direction::W),
                    _ => None,
                }
                .map(|d| ((x as isize - 1, y as isize - 1), vec![d]))
            })
        })
        .collect();
    let mut lines = input.lines();
    let width = lines.next().expect("at least one line").chars().count() - 2;
    let height = lines.count() - 1; // first and last are walls
    Valley {
        width: width as isize,
        height: height as isize,
        blizzards,
    }
}
