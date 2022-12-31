use std::collections::HashSet;
use std::fs;
use std::io;
use std::str::{self, FromStr};


#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
struct Item {
	item: char,
}

impl Item {
	fn new(item: char) -> Self {
		Self { item }
	}

	fn prio(&self) -> u64 {
		(match self.item.is_lowercase() {
			true => (self.item as u8) - 96,
			false => (self.item as u8) - 38,
		}) as u64
	}
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
struct Rucksack {
	c1: Vec<Item>,
	c2: Vec<Item>,
}

impl Rucksack {
	fn iter(&self) -> impl Iterator<Item = Item> + '_ {
		self.c1.iter().chain(self.c2.iter()).map(|i| *i)
	}

	fn inner_intersection(&self) -> Item {
		let c1 = HashSet::<&Item>::from_iter(self.c1.iter());
		let c2 = HashSet::<&Item>::from_iter(self.c2.iter());
		**(c1.intersection(&c2).next().unwrap())
	}

	fn intersection2(&self, other1: &Self, other2: &Self) -> Item {
		let c1 = HashSet::<Item>::from_iter(self.iter());
		let c2 = HashSet::<Item>::from_iter(other1.iter());
		let c3 = HashSet::<Item>::from_iter(other2.iter());

		let c12 = HashSet::<Item>::from_iter(c1.intersection(&c2)
			.into_iter().map(|i| *i));
		
		*c12.intersection(&c3).next().unwrap()
	}
}

impl str::FromStr for Rucksack {
	type Err = &'static str;
	fn from_str(s: &str) -> Result<Self, Self::Err> {
		assert!(s.len() % 2 == 0);
		let half = s.len() / 2;
		let c1 = &s[..half].chars().map(Item::new).collect::<Vec<_>>();
		let c2 = &s[half..].chars().map(Item::new).collect::<Vec<_>>();
		Ok(Self { c1: c1.to_vec(), c2: c2.to_vec() })
	}
}

fn main() -> io::Result<()> {
	let inp = fs::read_to_string("./input.txt")?;

	let rs = inp.split("\n")
		.filter(|line| !line.trim().is_empty())
		.map(Rucksack::from_str)
		.collect::<Result<Vec<_>, _>>()
		.unwrap();

	let sum = rs.iter()
		.map(|r| r.inner_intersection().prio())
		.sum::<u64>();

	println!("Part 1: {:?}", sum);

	let sum = &rs[..].chunks(3)
		.map(|grp| grp[0].intersection2(&grp[1], &grp[2]).prio())
		.sum::<u64>();

	println!("Part 2: {:?}", sum);

	Ok(())
}
