use std::str::FromStr;
use std::collections::VecDeque;
use std::num::ParseIntError;

use crate::Worry;
use crate::operator::Operator;
use crate::test::Test;

#[derive(Debug, Clone)]
pub struct Monkey {
	starting: VecDeque<Worry>,
	operator: Operator,
	test: Test,
}

impl Monkey {
	pub fn throw(&mut self, item: Worry) {
		self.starting.push_front(item);
	}

	pub fn remove(&mut self, idx: usize) -> Worry {
		self.starting.remove(idx).unwrap()
	}

	pub fn len(&self) -> usize {
		self.starting.len()
	}

	pub fn inspect(&self, item: Worry) -> Worry {
		match self.operator {
			Operator::Add(v) => item + v,
			Operator::Mul(v) => item * v,
			Operator::Pow(2) => item * item,
			_ => unreachable!(),
		}
	}

	pub fn test(&self, item: Worry) -> usize {
		match item % self.div() == 0 {
			true => self.test.if_true,
			false => self.test.if_false,
		}
	}

	pub fn div(&self) -> Worry {
		self.test.div
	}
}

impl FromStr for Monkey {
	type Err = ParseIntError;
	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let mut lines = s.split('\n');
		assert!(lines.next().unwrap().starts_with("Monkey "));

		let starting = lines.next().unwrap()
			.split(" items: ")
			.nth(1)
			.unwrap()
			.split(", ")
			.map(|n| n.parse::<Worry>())
			.collect::<Result<Vec<_>, _>>()
			.map(|mut v| {
				v.reverse();
				VecDeque::from_iter(v)
			})?;

		let operator = lines.next().unwrap().parse::<Operator>()?;
		let test = lines.collect::<Vec<_>>().join("\n").parse::<Test>()?;
		Ok(Self { starting, operator, test })
	}
}