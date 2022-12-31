use std::str;
use std::num::ParseIntError;

use crate::Worry;

#[derive(Debug, Clone, Copy)]
pub struct Test {
	pub div: Worry,
	pub if_true: usize,
	pub if_false: usize,
}

impl str::FromStr for Test {
	type Err = ParseIntError;
	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let mut lines = s.split('\n');

		let div = lines.next().unwrap().trim();
		assert!(div.starts_with("Test: divisible by"));
		let div = div.split(' ')
			.nth(3).unwrap()
			.parse::<Worry>()?;

		let if_true = lines.next().unwrap().trim();
		assert!(if_true.starts_with("If true: throw to monkey"));
		let if_true = if_true.split(' ')
			.nth(5).unwrap()
			.parse::<usize>()?;

		let if_false = lines.next().unwrap().trim();
		assert!(if_false.starts_with("If false: throw to monkey"));
		let if_false = if_false.split(' ')
			.nth(5).unwrap()
			.parse::<usize>()?;

		Ok(Self { div, if_true, if_false })
	}
}
