use std::fs;
use std::io;
use std::str;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum MatchResult {
	Win,
	Loss,
	Draw,
}

impl MatchResult {
	fn points(&self) -> u64 {
		match self {
			Self::Win => 6,
			Self::Draw => 3,
			Self::Loss => 0,
		}
	}

	fn rev(&self) -> MatchResult {
		match self {
			Self::Win => Self::Loss,
			Self::Loss => Self::Win,
			Self::Draw => Self::Draw,
		}
	}
}

impl str::FromStr for MatchResult {
	type Err = &'static str;
	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s {
			"X" => Ok(Self::Loss),
			"Y" => Ok(Self::Draw),
			"Z" => Ok(Self::Win),
			_ => Err("invalid shape"),
		}
	}
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Shape {
	Rock,
	Paper,
	Scissors,
}

impl str::FromStr for Shape {
	type Err = String;
	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s {
			"A" | "X" => Ok(Self::Rock),
			"B" | "Y" => Ok(Self::Paper),
			"C" | "Z" => Ok(Self::Scissors),
			_ => Err(format!("invalid shape: {}", s)),
		}
	}
}

impl Shape {
	fn points(&self) -> u64 {
		match self {
			Self::Rock => 1,
			Self::Paper => 2,
			Self::Scissors => 3,
		}
	}

	fn game(&self, other: &Self) -> MatchResult {
		match self {
			_ if self.win()  == *other => MatchResult::Win,
			_ if self.draw() == *other => MatchResult::Draw,
			_ if self.loss() == *other => MatchResult::Loss,
			_ => unreachable!(),
		}
	}

	fn game_points(&self, other: &Self) -> u64 {
		self.points() + self.game(other).points()
	}

	fn arrange_result(&self, r: &MatchResult) -> Shape {
		match r {
			MatchResult::Win => self.win(),
			MatchResult::Draw => self.draw(),
			MatchResult::Loss => self.loss(),
		}
	}

	fn win(&self) -> Shape {
		match self {
			Self::Rock => Self::Scissors,
			Self::Paper => Self::Rock,
			Self::Scissors => Self::Paper,
		}
	}

	fn draw(&self) -> Shape {
		*self
	}

	fn loss(&self) -> Shape {
		self.win().win()
	}
}

fn parse_line1(line: &str) -> (Shape, Shape) {
	let mut parts = line.split(' ');
	(
		parts.next().unwrap().parse::<Shape>().unwrap(),
		parts.next().unwrap().parse::<Shape>().unwrap(),
	)

}	

fn parse_line2(line: &str) -> (Shape, MatchResult) {
	let mut parts = line.split(' ');
	(
		parts.next().unwrap().parse::<Shape>().unwrap(),
		parts.next().unwrap().parse::<MatchResult>().unwrap(),
	)
}	

fn main() -> io::Result<()> {
	let inp = fs::read_to_string("./input.txt")?;

	let points = inp.split("\n")
		.filter(|line| !line.trim().is_empty())
		.map(parse_line1)
		.map(|(other, ours)| ours.game_points(&other))
		.sum::<u64>();

	println!("Part 1: {}", points);

	let points = inp.split("\n")
		.filter(|line| !line.trim().is_empty())
		.map(parse_line2)
		.map(|(other, res)|
			other.arrange_result(&res.rev()).points() + res.points())
		.sum::<u64>();

	println!("Part 2: {}", points);

	Ok(())
}
