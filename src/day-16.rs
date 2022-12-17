use cached::proc_macro::cached;
use std::{
    cmp::min,
    collections::{hash_map::DefaultHasher, BTreeMap, HashMap},
    fs::read_to_string,
    hash::Hasher,
};

fn main() {
    let input = read_to_string("inputs/day-16.txt").expect("missing input file");
    let mut valves = parse_input(input);
    reduce_valves(&mut valves);
    let res = release_pressure(&mut valves, &[(0, 30)]);
    println!("Part 1: {}", res);
    let res = release_pressure(
        &mut valves,
        &[(0, 26), (0, 26)],
    );
    println!("Part 2: {}", res);
}

#[derive(Debug)]
struct Valve {
    tunnels: HashMap<usize, usize>,
    flow_rate: usize,
    is_open: bool,
}

impl Valve {
    fn new(tunnels: HashMap<usize, usize>, flow_rate: usize) -> Self {
        Self {
            tunnels,
            flow_rate,
            is_open: false,
        }
    }
}

type Valves = BTreeMap<usize, Valve>; // this has deterministic ordering

fn cache_key(valves: &Valves, actors: &[(usize, usize)]) -> u64 {
    let mut hasher = DefaultHasher::new();
    for (k, v) in valves {
        hasher.write_u8(u8::from(v.is_open));
        for time in actors
            .iter()
            .map(|(tunnel, time)| if k == tunnel { time } else { &0 })
        {
            hasher.write_usize(*time);
        }
    }
    hasher.finish()
}

#[cached(key = "u64", convert = r##"{cache_key(&valves, actors)}"##)]
fn release_pressure(valves: &mut Valves, actors: &[(usize, usize)]) -> usize {
    let mut actors = actors.to_owned();
    actors.sort_by_key(|(_, time)| *time);
    let (tunnel, time_remaining) = actors.pop().expect("we have actors");

    if time_remaining == 0 {
        return 0;
    }

    let mut choices = vec![];

    let valve = &valves[&tunnel];
    let flow_rate = valve.flow_rate;
    if !valve.is_open && flow_rate > 0 {
        actors.push((tunnel, time_remaining - 1));
        valves
            .entry(tunnel)
            .and_modify(|e| e.is_open = true);
        choices.push((time_remaining - 1) * flow_rate + release_pressure(valves, &actors));
        // backtrack
        valves.entry(tunnel).and_modify(|e| e.is_open = false);
        actors.pop();
    } else {
        let tunnels = &valves[&tunnel].tunnels.clone();
        for (tunnel, distance) in tunnels {
            if time_remaining >= *distance {
                actors.push((*tunnel, time_remaining - distance));
                choices.push(release_pressure(valves, &actors));
                actors.pop();
            }
        }
    }

    *choices.iter().max().unwrap_or(&0)
}

fn parse_input(input: String) -> Valves {
    input.lines().map(parse_valve).collect()
}

fn parse_valve(input: &str) -> (usize, Valve) {
    let mut parts = input.split("; ");
    let mut valve_parts = parts.next().unwrap().split(' ');
    let id = tunnel_id(valve_parts.nth(1).unwrap());
    let flow_rate = valve_parts
        .nth(2)
        .unwrap()
        .split('=')
        .nth(1)
        .unwrap()
        .parse::<usize>()
        .unwrap();
    let tunnels = parts
        .next()
        .unwrap()
        .splitn(5, ' ')
        .last()
        .unwrap()
        .split(", ")
        .map(|s| (tunnel_id(s), 1))
        .collect();
    (id, Valve::new(tunnels, flow_rate))
}

fn reduce_valves(valves: &mut Valves) {
    while let Some(key) = valves
        .iter()
        .filter_map(|(k, v)| {
            if v.flow_rate == 0 && v.tunnels.len() == 2 {
                Some(k)
            } else {
                None
            }
        }).cloned()
        .next()
    {
        let removed = valves.remove(&key).expect("just checked this key");
        let connected_tunnels = removed.tunnels.keys().collect::<Vec<_>>();
        let full_distance = removed.tunnels.values().sum();
        valves
            .entry(*connected_tunnels[0])
            .and_modify(|e| {
                e.tunnels.remove(&key);
                let distance = e
                    .tunnels
                    .entry(*connected_tunnels[1])
                    .or_insert(full_distance);
                *distance = min(*distance, full_distance);
            });
        valves
            .entry(*connected_tunnels[1])
            .and_modify(|e| {
                e.tunnels.remove(&key);
                let distance = e
                    .tunnels
                    .entry(*connected_tunnels[0])
                    .or_insert(full_distance);
                *distance = min(*distance, full_distance);
            });
    }
}

fn tunnel_id(name: &str) -> usize {
    name.chars().fold(0, |acc, c| acc * 100 + c as usize - 'A' as usize)
}
