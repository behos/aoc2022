use std::fs::read_to_string;

fn main() {
    let input = read_to_string("inputs/day-25.txt").expect("missing input file");
    let total_fuel = total_fuel(input.trim());
    println!("{}", total_fuel);
}

fn total_fuel(input: &str) -> String {
    into_snafu(input.lines().map(from_snafu).sum())
}

fn from_snafu(input: &str) -> isize {
    input.chars().fold(0, |acc, c| {
        acc * 5
            + match c {
                '2' => 2,
                '1' => 1,
                '0' => 0,
                '-' => -1,
                '=' => -2,
                _ => panic!("unexpected symbol"),
            }
    })
}

fn into_snafu(mut num: isize) -> String {
    let mut chars = vec![];
    while num > 0 {
        let (c, carry) = match num % 5 {
            4 => ('-', 1),
            3 => ('=', 1),
            2 => ('2', 0),
            1 => ('1', 0),
            0 => ('0', 0),
            _ => panic!("uh oh! negatives!")
        };
        chars.push(c);
        num = num / 5 + carry;
    }
    chars.into_iter().rev().collect()
}
