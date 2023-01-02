use std::cmp::Ordering;
use std::fs;
use std::io::{self, ErrorKind};
use std::iter;
use std::str::FromStr;

fn split_with_offsets(st: &str, sep: char) -> Vec<(usize, String)> {
	let mut outer = Vec::new();
	let mut inner = Vec::new();
	let mut block_idx = 0;

	for (i, c) in st.chars().enumerate() {
		match c {
			c if c == sep => {
				outer.push((block_idx, String::from_iter(&inner)));
				inner.clear();
				block_idx = i + 1;
			}
			c if c.is_ascii_whitespace() => (),
			c => inner.push(c),
		}
	}

	if !inner.is_empty() {
		let s = String::from_iter(&inner);
		outer.push((block_idx, s));
	}

	outer
}

#[derive(Debug, Clone, Eq)]
enum Packet {
	Imm(u64),
	List(Vec<Packet>),
}

impl Packet {
	fn parse_node(st: &str) -> (usize, Packet) {
		let start_idx = st.chars().position(|c| c == '[').unwrap() + 1;
		let parts: Vec<_> = split_with_offsets(&st[start_idx..], ',')
			.into_iter()
			.map(|(v, w)| (v + start_idx, w))
			.collect();

		let mut i: usize = 0;
		let mut out = Vec::new();

		while i < parts.len() {
			let (cons, part) = &parts[i];
			match part {
				start if start.starts_with('[') => {
					let (cns, node) = Self::parse_node(&st[*cons..]);
					i = parts.iter().position(|(c, _)| *c > cons + cns)
						.unwrap_or(usize::MAX);
					out.push(node);
				}
				end if end.ends_with(']') => {
					let idx = end.chars().position(|c| c == ']').unwrap();
					if idx > 0 {
						let node = Self::Imm(end[..idx].parse::<u64>()
							.unwrap());
						out.push(node);
					}
					return (cons + idx, Self::List(out));
				}
				imm => {
					let node = Self::Imm(imm.parse::<u64>().unwrap());
					out.push(node);
					i += 1;
				}
			}
		}

		(st.len(), Self::List(out))
	}

	fn to_list(&self) -> Self {
		match self {
			Self::Imm(_) => Packet::List(vec![self.clone()]),
			_ => unreachable!(),
		}
	}
}

impl FromStr for Packet {
	type Err = std::convert::Infallible;
	fn from_str(st: &str) -> Result<Self, Self::Err> {
		Ok(Self::parse_node(st).1)
	}
}

impl PartialOrd for Packet {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		match (self, other) {
			(Packet::Imm(a), Packet::Imm(b)) => Some(a.cmp(b)),
			(Packet::List(a), Packet::List(b)) =>
				a.iter().zip(b)
					.map(|(f, s)| f.partial_cmp(s).unwrap())
					.find(|r| *r != Ordering::Equal)
					.or_else(|| Some(a.len().cmp(&b.len()))),
			(Packet::List(_), Packet::Imm(_)) =>
				self.partial_cmp(&other.to_list()),
			(Packet::Imm(_), Packet::List(_)) =>
				self.to_list().partial_cmp(other),
		}
	}
}

impl PartialEq for Packet {
	fn eq(&self, other: &Self) -> bool {
		self.partial_cmp(other) == Some(Ordering::Equal)
	}
}

struct PacketPair {
	first: Packet,
	second: Packet,
}

impl FromStr for PacketPair {
	type Err = io::Error;
	fn from_str(st: &str) -> Result<Self, Self::Err> {
		let mut parts = st.split('\n');
		let (first, second) = parts.next().zip(parts.next())
			.ok_or_else(|| io::Error::new(ErrorKind::InvalidData, st))?;
		let first = first.parse::<Packet>().unwrap();
		let second = second.parse::<Packet>().unwrap();
		Ok(Self { first, second })
	}
}

fn part1(pairs: &[PacketPair]) -> usize {
	pairs.iter()
		.enumerate()
		.filter(|(_, p)| p.first < p.second)
		.map(|(i, _)| i + 1)
		.sum()
}

fn part2(pairs: Vec<PacketPair>) -> usize {
	let p1 = "[[2]]".parse::<Packet>().unwrap();
	let p2 = "[[6]]".parse::<Packet>().unwrap();
	let mut packets: Vec<_> = pairs.into_iter()
		.flat_map(|p| [p.first, p.second])
		.chain(iter::once(p1.clone()))
		.chain(iter::once(p2.clone()))
		.collect();
	packets.sort_by(|a, b| a.partial_cmp(b).unwrap());

	packets.iter()
		.enumerate()
		.filter(|(_, p)| **p == p1 || **p == p2)
		.map(|(i, _)| i + 1)
		.product()
}

fn main() -> io::Result<()> {
	let inp = fs::read_to_string("./input.txt")?;

	let pairs = inp.split("\n\n")
		.filter(|p| !p.trim().is_empty())
		.map(PacketPair::from_str)
		.collect::<Result<Vec<_>, _>>()?;

	println!("Part 1: {}", part1(&pairs));
	println!("Part 2: {}", part2(pairs));

	Ok(())
}
