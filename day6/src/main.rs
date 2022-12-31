use std::fs;
use std::io;
use std::collections::HashSet;

fn all_distinct(g: &[u8]) -> bool {
	g.len() == HashSet::<&u8>::from_iter(g).len()
}

fn find_marker(u: usize, s: &str) -> usize {
	s.as_bytes()
		.windows(u)
		.enumerate()
		.find(|(_, g)| all_distinct(g))
		.map(|(i, g)| i + g.len())
		.unwrap()
}

fn main() -> io::Result<()> {
	let inp = fs::read_to_string("./input.txt")?;

	let part1 = find_marker(4, &inp);
	println!("Part 1: {}", part1);

	let part2 = find_marker(14, &inp);
	println!("Part 2: {}", part2);

	Ok(())
}
