use std::{collections::HashMap, fs::read_to_string};

const HUMAN: &str = "humn";
const ROOT: &str = "root";

fn main() {
    let input = read_to_string("inputs/day-21.txt").expect("missing input file");
    let monkeys = parse::input(input);
    let root_yell = do_yell(&monkeys, ROOT);
    println!("Part 1: {}", root_yell);

    let human_yell = solve_for_human(&monkeys, ROOT, 0);
    println!("Part 2: {}", human_yell);
}

type Monkeys = HashMap<String, MonkeyBusiness>;

#[derive(Debug, Clone, Copy)]
enum Op {
    Add,
    Sub,
    Mul,
    Div,
}

impl Op {
    fn from(raw: char) -> Self {
        match raw {
            '*' => Op::Mul,
            '/' => Op::Div,
            '+' => Op::Add,
            '-' => Op::Sub,
            _ => panic!("impossible"),
        }
    }

    fn calculate(&self, lhs: i64, rhs: i64) -> i64 {
        match self {
            Op::Add => lhs + rhs,
            Op::Sub => lhs - rhs,
            Op::Mul => lhs * rhs,
            Op::Div => lhs / rhs,
        }
    }

    fn inverse(&self) -> Self {
        match self {
            Op::Add => Op::Sub,
            Op::Sub => Op::Add,
            Op::Mul => Op::Div,
            Op::Div => Op::Mul,
        }
    }

    // In cases where human is in the right hand side of an operation, we want to take
    // some special action.
    fn calculate_inverse(&self, lhs: i64, rhs: i64) -> i64 {
        match self {
            Op::Add => rhs - lhs,
            Op::Sub => lhs - rhs,
            Op::Mul => rhs / lhs,
            Op::Div => lhs / rhs,
        }
    }
}

#[derive(Debug)]
enum MonkeyBusiness {
    Yell(i64),
    YellOp(String, String, Op),
}

fn do_yell(monkeys: &Monkeys, name: &str) -> i64 {
    match &monkeys[name] {
        MonkeyBusiness::Yell(num) => *num,
        MonkeyBusiness::YellOp(lhs, rhs, op) => {
            op.calculate(do_yell(monkeys, lhs), do_yell(monkeys, rhs))
        }
    }
}

fn solve_for_human(monkeys: &Monkeys, root: &str, result: i64) -> i64 {
    let (root_lhs, root_rhs, op) = match &monkeys[root] {
        MonkeyBusiness::YellOp(lhs, rhs, op) => (lhs, rhs, op),
        MonkeyBusiness::Yell(_) => return result,
    };

    let (new_root, other, result) = {
        if contains_human(monkeys, root_lhs) {
            let rhs = do_yell(monkeys, root_rhs);
            (root_lhs, rhs, op.inverse().calculate(result, rhs))
        } else {
            let lhs = do_yell(monkeys, root_lhs);
            (root_rhs, lhs, op.calculate_inverse(lhs, result))
        }
    };

    solve_for_human(monkeys, new_root, if root == ROOT { other } else { result })
}

fn contains_human(monkeys: &Monkeys, name: &str) -> bool {
    name == HUMAN
        || match &monkeys[name] {
            MonkeyBusiness::YellOp(first, second, _) => {
                first == HUMAN
                    || second == HUMAN
                    || contains_human(monkeys, first)
                    || contains_human(monkeys, second)
            }
            MonkeyBusiness::Yell(_) => false,
        }
}

mod parse {

    use super::{MonkeyBusiness, Monkeys, Op};

    use nom::{
        branch::alt,
        bytes::complete::tag,
        character::complete::{alpha1, i64, newline, one_of, space1},
        multi::separated_list0,
        sequence::{delimited, terminated, tuple},
        IResult as NomResult,
    };

    pub(crate) fn input(input: String) -> Monkeys {
        monkeys(&input)
            .expect("should be parseable")
            .1
            .into_iter()
            .map(|(nstr, val)| (nstr.to_string(), val))
            .collect()
    }

    fn monkeys(input: &str) -> NomResult<&str, Vec<(&str, MonkeyBusiness)>> {
        separated_list0(newline, monkey)(input)
    }

    fn monkey(input: &str) -> NomResult<&str, (&str, MonkeyBusiness)> {
        tuple((terminated(alpha1, tag(": ")), monkey_business))(input)
    }

    fn monkey_business(input: &str) -> NomResult<&str, MonkeyBusiness> {
        alt((yell_operator, yell_num))(input)
    }

    fn yell_operator(input: &str) -> NomResult<&str, MonkeyBusiness> {
        tuple((alpha1, delimited(space1, op, space1), alpha1))(input).map(
            |(input, (first, op, second))| {
                (
                    input,
                    MonkeyBusiness::YellOp(first.to_string(), second.to_string(), op),
                )
            },
        )
    }

    fn yell_num(input: &str) -> NomResult<&str, MonkeyBusiness> {
        i64(input).map(|(input, num)| (input, MonkeyBusiness::Yell(num)))
    }

    fn op(input: &str) -> NomResult<&str, Op> {
        one_of("*/+-")(input).map(|(input, op_str)| (input, Op::from(op_str)))
    }
}
