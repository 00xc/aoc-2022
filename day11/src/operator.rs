use std::str;
use std::num::ParseIntError;

use crate::Worry;

#[derive(Debug, Clone, Copy)]
pub enum Operator {
	Add(Worry),
	Mul(Worry),
	Pow(usize),
}

impl str::FromStr for Operator {
	type Err = ParseIntError;
	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let fields = s.trim();
		assert!(fields.starts_with("Operation: new = old"));

		let mut fields = fields.split(' ').skip(4);
		match fields.next().unwrap() {
			"*" => match fields.next().unwrap() {
				"old" => Ok(Self::Pow(2)),
				imm => Ok(Self::Mul(imm.parse::<Worry>()?))
			},
			"+" => match fields.next().unwrap() {
				"old" => Ok(Self::Mul(2)),
				imm => Ok(Self::Add(imm.parse::<Worry>()?))
			},
			_ => todo!()
		}
	}
}