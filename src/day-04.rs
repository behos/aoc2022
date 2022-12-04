use std::fs::read_to_string;

fn main() {
    let input = read_to_string("inputs/day-04.txt").expect("missing input file");
    let assignments: Vec<_> = input
        .split('\n')
        .filter_map(|s| match s {
            "" => None,
            s => {
                let mut pair = s.split(',').map(Assignment::from);
                Some((
                    pair.next().expect("should have one"),
                    pair.next().expect("should have second"),
                ))
            }
        })
        .collect();
    println!("Part 1: {}", count_containing(&assignments));
    println!("Part 2: {}", count_overlapping(&assignments));
}

struct Assignment {
    from: usize,
    to: usize,
}

impl From<&str> for Assignment {
    fn from(input: &str) -> Self {
        let mut parts = input.split('-');
        Self {
            from: parts
                .next()
                .expect("should have 'from' assignment")
                .parse::<usize>()
                .expect("should be number"),
            to: parts
                .next()
                .expect("should have 'to' assignment")
                .parse::<usize>()
                .expect("should be number"),
        }
    }
}

impl Assignment {
    fn contains(&self, other: &Self) -> bool {
        self.from <= other.from && self.to >= other.to
    }

    fn overlaps(&self, other: &Self) -> bool {
        !(self.from > other.to || self.to < other.from)
    }
}

type Pair = (Assignment, Assignment);

fn count_containing(assignments: &[Pair]) -> usize {
    assignments.iter().filter(|(a1, a2)| a1.contains(a2) || a2.contains(a1)).count()
}

fn count_overlapping(assignments: &[Pair]) -> usize {
    assignments.iter().filter(|(a1, a2)| a1.overlaps(a2)).count()
}
