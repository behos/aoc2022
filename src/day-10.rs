use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let file = File::open("inputs/day-10.txt").expect("missing input file");
    let operations = BufReader::new(file).lines().map(|res| {
        res.map(Operation::from_line)
            .expect("should not fail reading line")
    });
    let mut cpu = Cpu::new(operations);
    let mut crt = Crt::new();
    let data_points = [20, 60, 100, 140, 180, 220];
    let mut sig_sum = 0;
    for _ in 0..240 {
        cpu.tick();
        crt.draw(cpu.clock, cpu.register);
        if data_points.contains(&cpu.clock) {
            sig_sum += cpu.clock as i32 * cpu.register;
        }
    }
    println!("Part 1: {}", sig_sum);
    println!("Part 2: {}", crt.output());
}

const HEIGHT: usize = 6;
const WIDTH: usize = 40;

#[derive(Clone, Copy, Debug)]
enum Operation {
    AddX(i32),
    Noop,
}

impl Operation {
    fn ticks(&self) -> usize {
        match self {
            Self::Noop => 0,
            Self::AddX(_) => 1,
        }
    }

    fn from_line(line: String) -> Self {
        match line.split(' ').collect::<Vec<_>>()[..] {
            ["noop"] => Self::Noop,
            ["addx", amount] => Self::AddX(amount.parse::<i32>().expect("should be num")),
            _ => panic!("unrecognized op"),
        }
    }
}

struct Cpu<T: Iterator<Item = Operation>> {
    clock: usize,
    register: i32,
    operation: Option<(Operation, usize)>,
    stream: T,
}

impl<T: Iterator<Item = Operation>> Cpu<T> {
    fn new(stream: T) -> Self {
        Self {
            clock: 0,
            register: 1,
            operation: None,
            stream,
        }
    }

    fn tick(&mut self) {
        self.operation = match self.operation.take() {
            None => self.next_operation(),
            Some((op, 0)) => {
                self.do_operation(op);
                self.next_operation()
            }
            Some((op, num)) => Some((op, num - 1)),
        };
        self.clock += 1;
    }

    fn do_operation(&mut self, op: Operation) {
        if let Operation::AddX(num) = op {
            self.register += num
        }
    }

    fn next_operation(&mut self) -> Option<(Operation, usize)> {
        self.stream.next().map(|op| (op, op.ticks()))
    }
}

struct Crt {
    display: [[char; WIDTH]; HEIGHT],
}

impl Crt {
    fn new() -> Self {
        Self {
            display: [['.'; WIDTH]; HEIGHT],
        }
    }

    fn draw(&mut self, clock: usize, sprite_pos: i32) {
        let row = (clock - 1) / WIDTH;
        let col = (clock - 1) % WIDTH;
        if sprite_pos - 1 <= col as i32 && col as i32 <= sprite_pos + 1 {
            self.display[row][col] = '#';
        }
    }

    fn output(&self) -> String {
        self.display
            .iter()
            .map(|row| row.iter().collect::<String>())
            .fold(String::new(), |acc, row| acc + "\n" + &row)
    }
}
