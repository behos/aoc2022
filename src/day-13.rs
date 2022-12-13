use std::{cmp::Ordering, fs::read_to_string};

fn main() {
    let divider_packets = [
        Packet::List(vec![Packet::List(vec![Packet::Integer(2)])]),
        Packet::List(vec![Packet::List(vec![Packet::Integer(6)])]),
    ];

    let divider_packets_lookup = [ // I don't want to derive Clone for packet :P
        Packet::List(vec![Packet::List(vec![Packet::Integer(2)])]),
        Packet::List(vec![Packet::List(vec![Packet::Integer(6)])]),
    ];

    let input = read_to_string("inputs/day-13.txt").expect("missing input file");
    let pairs = parse::input(input);
    let order_sum = pairs
        .iter()
        .enumerate()
        .filter_map(|(i, (left, right))| if left < right { Some(i + 1) } else { None })
        .sum::<usize>();
    println!("Part 1: {}", order_sum);
    let mut all_packets = pairs.into_iter().flat_map(|(p1, p2)| [p1, p2]).collect::<Vec<_>>();
    all_packets.extend(divider_packets.into_iter());
    all_packets.sort();
    let decoder_key = all_packets
        .iter()
        .enumerate()
        .filter_map(|(i, packet)| {
            if divider_packets_lookup.contains(packet) {
                Some(i + 1)
            } else {
                None
            }
        })
        .product::<usize>();
    println!("Part 2: {}", decoder_key);
}

type Pair = (Packet, Packet);

#[derive(Debug, Eq, PartialEq)]
enum Packet {
    List(Vec<Packet>),
    Integer(u32),
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Packet::Integer(left), Packet::Integer(right)) => left.cmp(right),
            (Packet::List(left), Packet::List(right)) => left.cmp(right),
            (Packet::Integer(left), Packet::List(right)) => {
                ([Packet::Integer(*left)].as_slice()).cmp(right)
            }
            (Packet::List(left), Packet::Integer(right)) => {
                left.as_slice().cmp(&[Packet::Integer(*right)])
            }
        }
    }
}

mod parse {
    use super::{Packet, Pair};
    use nom::{
        branch::alt,
        bytes::complete::tag,
        character::complete::{newline, u32},
        multi::separated_list0,
        IResult as NomResult, Parser,
    };

    pub(crate) fn input(input: String) -> Vec<Pair> {
        pairs(&input).expect("should be parseable").1
    }

    fn pairs(input: &str) -> NomResult<&str, Vec<Pair>> {
        let (input, pairs) = separated_list0(newline.and(newline), pair)(input)?;
        Ok((input, pairs))
    }

    fn pair(input: &str) -> NomResult<&str, Pair> {
        let (input, p1) = packet(input)?;
        let (input, _) = newline(input)?;
        let (input, p2) = packet(input)?;
        Ok((input, (p1, p2)))
    }

    fn packet(input: &str) -> NomResult<&str, Packet> {
        alt((list, integer))(input)
    }

    fn list(input: &str) -> NomResult<&str, Packet> {
        let (input, _) = tag("[")(input)?;
        let (input, packets) = separated_list0(tag(","), packet)(input)?;
        let (input, _) = tag("]")(input)?;
        Ok((input, Packet::List(packets)))
    }

    fn integer(input: &str) -> NomResult<&str, Packet> {
        let (input, num) = u32(input)?;
        Ok((input, Packet::Integer(num)))
    }
}
