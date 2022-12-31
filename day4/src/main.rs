use std::io;
use std::fs;
use std::str;

#[derive(Debug)]
struct Range {
	start: u64,
	end: u64,
}

impl Range {
	fn contains(&self, other: &Self) -> bool {
		self.start <= other.start && self.end >= other.end
	}

	fn overlaps(&self, other: &Self) -> bool {
		self.contains(other) ||
			other.start <= self.start && self.start <= other.end ||
			other.start <= self.end && self.start <= other.end
	}
}

impl str::FromStr for Range {
	type Err = io::Error;
	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let mut parts = s.split('-');
		let start = parts.next().unwrap().parse::<u64>().unwrap();
		let end = parts.next().unwrap().parse::<u64>().unwrap();
		Ok(Self { start, end })
	}
}

fn parse_line(s: &str) -> io::Result<(Range, Range)> {
	let mut parts = s.split(',');
	let p1 = parts.next().unwrap().parse::<Range>()?;
	let p2 = parts.next().unwrap().parse::<Range>()?;
	Ok((p1, p2))
}

fn main() -> io::Result<()> {
	let inp = fs::read_to_string("./input.txt")?;

	let rs = inp.split("\n")
		.filter(|line| !line.trim().is_empty())
		.map(parse_line)
		.collect::<Result<Vec<_>, _>>()?;

	let num_contained = rs.iter()
		.filter(|(a, b)| a.contains(&b) || b.contains(&a))
		.count();

	println!("Part 1: {:?}", num_contained);

	let num_overlaps = rs.iter()
		.filter(|(a, b)| a.overlaps(&b) || b.overlaps(&a))
		.count();

	println!("Part 2: {:?}", num_overlaps);

	Ok(())
}
