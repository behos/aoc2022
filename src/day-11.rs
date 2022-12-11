fn main() {
    #[rustfmt::skip]
    let mut monkeys_p1 = [
        Monkey::new(vec![91, 58, 52, 69, 95, 54], |old| old * 13, 7, 1, 5),
        Monkey::new(vec![80, 80, 97, 84], |old| old * old, 3, 3, 5),
        Monkey::new(vec![86, 92, 71], |old| old + 7, 2, 0, 4),
        Monkey::new(vec![96, 90, 99, 76, 79, 85, 98, 61], |old| old + 4, 11, 7, 6),
        Monkey::new(vec![60, 83, 68, 64, 73], |old| old * 19, 17, 1, 0),
        Monkey::new(vec![96, 52, 52, 94, 76, 51, 57], |old| old + 3, 5, 7, 3),
        Monkey::new(vec![75], |old| old + 5, 13, 4, 2),
        Monkey::new(vec![83, 75], |old| old + 1, 19, 2, 6),
    ];
    let mut monkeys_p2 = monkeys_p1.clone();

    play_rounds(&mut monkeys_p1, 20, 3);
    println!("Part 1: {}", most_active_product(&monkeys_p1));
    play_rounds(&mut monkeys_p2, 10_000, 1);
    println!("Part 2: {}", most_active_product(&monkeys_p2));
}

// just some aliases to determine what each usize is;
type Worry = usize;
type Id = usize;
type Throw = (Worry, Id);

#[derive(Clone)]
struct Monkey {
    items: Vec<Worry>,
    divisible_by: usize,
    if_true: Id,
    if_false: Id,
    operation: fn(Worry) -> Worry,
    inspections: usize,
}

impl Monkey {
    fn new(
        items: Vec<Worry>,
        operation: fn(Worry) -> Worry,
        divisible_by: usize,
        if_true: Id,
        if_false: Id,
    ) -> Self {
        Self {
            items,
            divisible_by,
            if_false,
            if_true,
            operation,
            inspections: 0,
        }
    }

    fn take_turn(&mut self, stress_relief: usize, worry_factor: usize) -> Vec<Throw> {
        self.inspections += self.items.len();
        self.items
            .drain(..)
            .map(|worry| {
                let new_worry = (self.operation)(worry) / stress_relief % worry_factor;
                let new_id = if new_worry % self.divisible_by == 0 {
                    self.if_true
                } else {
                    self.if_false
                };
                (new_worry, new_id)
            })
            .collect()
    }

    fn take_worry(&mut self, worry: Worry) {
        self.items.push(worry);
    }
}

fn play_rounds(monkeys: &mut [Monkey; 8], rounds: usize, stress_relief: usize) {
    let worry_factor = monkeys.iter().map(|m| m.divisible_by).product();
    for _ in 0..rounds {
        for i in 0..8 {
            let monkey = &mut monkeys[i];
            let throws = monkey.take_turn(stress_relief, worry_factor);
            for (worry, id) in throws {
                monkeys[id].take_worry(worry);
            }
        }
    }
}

fn most_active_product(monkeys: &[Monkey; 8]) -> usize {
    let mut inspections = monkeys.iter().map(|m| m.inspections).collect::<Vec<_>>();
    inspections.sort();
    inspections[7] * inspections[6]
}
