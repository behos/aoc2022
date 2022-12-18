use std::{
    collections::{BTreeSet, HashMap, HashSet},
    fs::read_to_string,
};

fn main() {
    let views = read_to_string("inputs/day-18.txt")
        .expect("missing input file")
        .trim()
        .lines()
        .map(to_cube)
        .collect();

    let area = count_area(&views);
    let air_pockets = get_air_pockets(&views);
    let air_pockets_area = count_area(&air_pockets);
    println!("Part 1: {}", area);
    println!("Part 2: {}", area - air_pockets_area);
}

type Cube = (usize, usize, usize);

fn to_cube(raw: &str) -> Cube {
    let mut parts = raw.split(',');
    (
        parts.next().unwrap().parse().unwrap(),
        parts.next().unwrap().parse().unwrap(),
        parts.next().unwrap().parse().unwrap(),
    )
}

type Plane = HashMap<(usize, usize), BTreeSet<usize>>;
#[derive(Debug)]
struct Views {
    xy: Plane,
    xz: Plane,
    yz: Plane,
}

impl Views {
    fn new() -> Self {
        Self {
            xy: HashMap::new(),
            xz: HashMap::new(),
            yz: HashMap::new(),
        }
    }
}

impl FromIterator<Cube> for Views {
    fn from_iter<T>(iter: T) -> Self
    where
        T: IntoIterator<Item = Cube>,
    {
        let mut views = Views::new();

        for (x, y, z) in iter {
            views.xy.entry((x, y)).or_default().insert(z);
            views.xz.entry((x, z)).or_default().insert(y);
            views.yz.entry((y, z)).or_default().insert(x);
        }

        views
    }
}

fn count_area(views: &Views) -> usize {
    // the area is the sum of all the gaps between cubes in each view.
    // .... unless there are air holes in the object :/
    [&views.xy, &views.xz, &views.yz]
        .into_iter()
        .map(count_plane_area)
        .sum()
}

fn count_plane_area(plane: &Plane) -> usize {
    plane
        .iter()
        .map(|(_, points)| {
            let mut count = 0;
            let mut last_point = None;
            for point in points {
                match last_point {
                    Some(lpoint) if lpoint == point - 1 => {}
                    None => count += 1,    // entry point of new surface
                    Some(_) => count += 2, // exit point of previous + entry point of new
                }
                last_point = Some(*point);
            }
            if last_point.is_some() {
                count += 1; // exit point of last
            }
            count
        })
        .sum()
}

fn get_air_pockets(views: &Views) -> Views {
    // get the air pockets by getting all the empty points per area and removing all the ones
    // that are visible from the outer side of any plane.
    let mut candidate_unreachable_cubes = get_invisible_cubes(views);
    let mut reachable_cubes = get_outer_cubes(views);
    let mut unreachable_len = 0;
    while unreachable_len != candidate_unreachable_cubes.len() {
        unreachable_len = candidate_unreachable_cubes.len();
        let expanded_reachable: HashSet<_> = reachable_cubes.iter().flat_map(expand_cube).collect();
        let new_reachable = expanded_reachable.intersection(&candidate_unreachable_cubes);
        reachable_cubes.extend(new_reachable);
        candidate_unreachable_cubes = candidate_unreachable_cubes
            .difference(&reachable_cubes)
            .cloned()
            .collect();
    }
    candidate_unreachable_cubes.into_iter().collect()
}

fn get_invisible_cubes(views: &Views) -> HashSet<Cube> {
    let xy_gaps = get_gaps(&views.xy, |(x, y, z)| (x, y, z));
    let xz_gaps = get_gaps(&views.xz, |(x, z, y)| (x, y, z));
    let yz_gaps = get_gaps(&views.yz, |(y, z, x)| (x, y, z));
    let invisible_cubes = xy_gaps
        .intersection(&xz_gaps)
        .cloned()
        .collect::<HashSet<_>>();
    invisible_cubes.intersection(&yz_gaps).cloned().collect()
}

fn get_gaps(plane: &Plane, translator: fn(Cube) -> Cube) -> HashSet<Cube> {
    plane
        .iter()
        .flat_map(|((x, y), points)| {
            let mut missing = vec![];
            let mut last_point: Option<usize> = None;
            for point in points {
                match last_point {
                    None => {}
                    Some(lpoint) => {
                        missing.extend(((lpoint + 1)..*point).map(|p| translator((*x, *y, p))))
                    }
                }
                last_point = Some(*point);
            }
            missing
        })
        .collect()
}

fn get_outer_cubes(views: &Views) -> HashSet<Cube> {
    let xy_outer = get_outer(&views.xy, |(x, y, z)| (x, y, z));
    let xz_outer = get_outer(&views.xz, |(x, z, y)| (x, y, z));
    let yz_outer = get_outer(&views.yz, |(y, z, x)| (x, y, z));
    let outer_cubes = xy_outer.union(&xz_outer).cloned().collect::<HashSet<_>>();
    outer_cubes
        .union(&yz_outer)
        .cloned()
        .collect::<HashSet<_>>()
}

fn get_outer(plane: &Plane, translator: fn(Cube) -> Cube) -> HashSet<Cube> {
    let plane_max = plane
        .iter()
        .map(|(_, s)| s.last().expect("should be a point"))
        .max().expect("no way there's no point in the whole plane");
    plane
        .iter()
        .flat_map(|((x, y), points)| {
            (0..*points.first().expect("must be a point"))
                .chain(*points.last().expect("must be a point") + 1..=plane_max + 1)
                .map(|p| translator((*x, *y, p)))
        })
        .collect()
}

fn expand_cube((x, y, z): &Cube) -> [Cube; 7] {
    [
        (*x, *y, *z),
        (x.saturating_sub(1), *y, *z),
        (x + 1, *y, *z),
        (*x, y.saturating_sub(1), *z),
        (*x, y + 1, *z),
        (*x, *y, z.saturating_sub(1)),
        (*x, *y, z + 1),
    ]
}
