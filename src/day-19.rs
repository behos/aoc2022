use std::{
    cmp::max,
    collections::hash_map::DefaultHasher,
    fs::read_to_string,
    hash::{Hash, Hasher},
    ops::{Add, Sub},
};

use cached::proc_macro::cached;

fn main() {
    let blueprints = read_to_string("inputs/day-19.txt")
        .expect("missing input file")
        .trim()
        .lines()
        .map(to_blueprint)
        .collect::<Vec<_>>();

    let quality_levels: usize = blueprints
        .iter()
        .enumerate()
        .map(|(i, blueprint)| (i + 1) * max_geodes(Factory::new(*blueprint), 24))
        .sum();

    println!("Part 1: {}", quality_levels);

    let geode_product: usize = blueprints[..3]
        .iter()
        .map(|blueprint| max_geodes(Factory::new(*blueprint), 32))
        .product();

    println!("Part 2: {}", geode_product);
}

macro_rules! maybe_build {
    ($factory:ident, $type:ident, $choices:ident, $upcoming_supply:expr, $time_limit:expr) => {
        if $factory.can_build($factory.blueprint.$type)
            && $factory.robots.$type < $factory.max_robots.$type
        {
            $choices.push(max_geodes(
                $factory.produce($factory.blueprint.$type, Supply::$type(1), $upcoming_supply),
                $time_limit - 1,
            ))
        }
    };
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Blueprint {
    ore: Supply,
    clay: Supply,
    obsidian: Supply,
    geode: Supply,
}

#[derive(Default, Debug, Clone, Copy, PartialOrd, PartialEq, Eq, Hash)]
struct Supply {
    ore: usize,
    clay: usize,
    obsidian: usize,
    geode: usize,
}

impl Supply {
    fn ore(amount: usize) -> Self {
        Supply {
            ore: amount,
            ..Default::default()
        }
    }

    fn clay(amount: usize) -> Self {
        Supply {
            clay: amount,
            ..Default::default()
        }
    }

    fn obsidian(amount: usize) -> Self {
        Supply {
            obsidian: amount,
            ..Default::default()
        }
    }

    fn geode(amount: usize) -> Self {
        Supply {
            geode: amount,
            ..Default::default()
        }
    }
}

impl Add<Supply> for Supply {
    type Output = Supply;

    fn add(self, rhs: Supply) -> Self {
        Self {
            ore: self.ore + rhs.ore,
            clay: self.clay + rhs.clay,
            obsidian: self.obsidian + rhs.obsidian,
            geode: self.geode + rhs.geode,
        }
    }
}

impl Sub<Supply> for Supply {
    type Output = Supply;

    fn sub(self, rhs: Supply) -> Self {
        Self {
            ore: self.ore - rhs.ore,
            clay: self.clay - rhs.clay,
            obsidian: self.obsidian - rhs.obsidian,
            geode: self.geode - rhs.geode,
        }
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Debug, Hash)]
struct Factory {
    blueprint: Blueprint,
    warehouse: Supply,
    robots: Supply,
    max_robots: Supply,
}

impl Factory {
    fn new(blueprint: Blueprint) -> Self {
        let max_robots = [
            blueprint.ore,
            blueprint.clay,
            blueprint.obsidian,
            blueprint.geode,
        ]
        .into_iter()
        .reduce(|acc, s| Supply {
            ore: max(acc.ore, s.ore),
            clay: max(acc.clay, s.clay),
            obsidian: max(acc.obsidian, s.obsidian),
            geode: max(acc.geode, s.geode),
        })
        .expect("we certainly have something");

        Self {
            blueprint,
            warehouse: Supply::default(),
            robots: Supply {
                ore: 1,
                ..Default::default()
            },
            max_robots,
        }
    }

    fn can_build(&self, recipe: Supply) -> bool {
        self.warehouse.ore >= recipe.ore
            && self.warehouse.clay >= recipe.clay
            && self.warehouse.obsidian >= recipe.obsidian
            && self.warehouse.geode >= recipe.geode
    }

    fn produce(
        &self,
        used_materials: Supply,
        produced_robots: Supply,
        new_materials: Supply,
    ) -> Factory {
        Factory {
            warehouse: self.warehouse - used_materials + new_materials,
            robots: self.robots + produced_robots,
            ..*self
        }
    }
}

fn cache_key(factory: Factory, time_limit: usize) -> u64 {
    let mut hasher = DefaultHasher::new();
    factory.hash(&mut hasher);
    hasher.write_usize(time_limit);
    hasher.finish()
}

#[cached(key = "u64", convert = r##"{cache_key(factory, time_limit)}"##)]
fn max_geodes(factory: Factory, time_limit: usize) -> usize {
    // each robot will fetch a thing of its type. but we can't use it this turn.
    let upcoming_supply = factory.robots;

    if time_limit == 1 {
        return factory.warehouse.geode + upcoming_supply.geode;
    }

    let mut choices = vec![];

    // your choices are basically to either build a robot or do nothing.
    // if we can build a geode robot, build it. It's always the best choice
    if factory.can_build(factory.blueprint.geode) {
        return max_geodes(
            factory.produce(factory.blueprint.geode, Supply::geode(1), upcoming_supply),
            time_limit - 1,
        );
    }

    maybe_build!(factory, obsidian, choices, upcoming_supply, time_limit);
    maybe_build!(factory, clay, choices, upcoming_supply, time_limit);
    maybe_build!(factory, ore, choices, upcoming_supply, time_limit);

    choices.push(max_geodes(
        factory.produce(Supply::default(), Supply::default(), upcoming_supply),
        time_limit - 1,
    ));

    choices.into_iter().max().unwrap_or(0)
}

fn to_blueprint(input: &str) -> Blueprint {
    let mut parts = input.split(": ").nth(1).unwrap().split('.');
    Blueprint {
        ore: to_supply(parts.next().unwrap()),
        clay: to_supply(parts.next().unwrap()),
        obsidian: to_supply(parts.next().unwrap()),
        geode: to_supply(parts.next().unwrap()),
    }
}

fn to_supply(input: &str) -> Supply {
    input
        .split("costs ")
        .nth(1)
        .unwrap()
        .split(" and ")
        .map(|cost| {
            let mut cost_parts = cost.split(' ');
            let amount = cost_parts.next().unwrap().parse::<usize>().unwrap();
            match cost_parts.next().unwrap() {
                "ore" => Supply::ore(amount),
                "clay" => Supply::clay(amount),
                "obsidian" => Supply::obsidian(amount),
                "geode" => Supply::geode(amount),
                _ => panic!("dunno what that is"),
            }
        })
        .reduce(|s1, s2| s1 + s2)
        .unwrap()
}
