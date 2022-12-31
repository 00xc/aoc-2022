use std::fs;
use std::io::{self, ErrorKind};
use std::str::FromStr;

mod monkey;
mod operator;
mod test;

type Worry = u64;

#[derive(Clone)]
struct Monkeys(Vec<monkey::Monkey>);

impl Monkeys {
	fn len(&self) -> usize {
		self.0.len()
	}

	fn emulate<const ROUNDS: usize, const DIV: Worry>(&mut self)
		-> Vec<usize>
	{
		let mut inspected = vec![0; self.len()];
		let modulo = self.0.iter().map(|m| m.div()).product::<Worry>();

		for _ in 0..ROUNDS {
			for (src, insp) in inspected.iter_mut().enumerate() {
				let num_items = self.0[src].len();
				*insp += num_items;

				for item_idx in (0..num_items).rev() {
					let monkey = &mut self.0[src];
					let item = monkey.remove(item_idx);
					let item = (monkey.inspect(item) / DIV) % modulo;
					let dst = monkey.test(item);
					self.0[dst].throw(item);
				}
			}
		}

		inspected
	}
}

impl FromStr for Monkeys {
	type Err = <monkey::Monkey as FromStr>::Err;
	fn from_str(st: &str) -> Result<Self, Self::Err> {
		st.split("\n\n")
			.map(monkey::Monkey::from_str)
			.collect::<Result<Vec<_>, _>>()
			.map(Self)
	}
}

fn part1(mut m: Monkeys) -> usize {
	let mut act = m.emulate::<20, 3>();
	act.sort();
	act.iter()
		.rev()
		.take(2)
		.product()
}

fn part2(mut m: Monkeys) -> usize {
	let mut act = m.emulate::<10000, 1>();
	act.sort();
	act.iter()
		.rev()
		.take(2)
		.product()
}

fn main() -> io::Result<()> {
	let inp = fs::read_to_string("./input.txt")?;
	let monkeys = inp.parse::<Monkeys>()
		.map_err(|e| io::Error::new(ErrorKind::InvalidData, e))?;

	println!("Part 1: {}", part1(monkeys.clone()));
	println!("Part 2: {}", part2(monkeys));

	Ok(())
}