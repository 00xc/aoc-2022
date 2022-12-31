use std::io::{self, ErrorKind};
use std::fs;
use std::str;
use std::fmt::{self, Write};
use std::num::ParseIntError;

enum Instruction {
	Noop,
	AddX(i64),
}

impl Instruction {
	fn num_cycles(&self) -> usize {
		match self {
			Self::Noop => 1,
			Self::AddX(_) => 2,
		}
	}
}

impl str::FromStr for Instruction {
	type Err = ParseIntError;
	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let mut fields = s.split(' ');
		match fields.next() {
			Some("addx") => fields.next()
				.unwrap()
				.parse::<i64>()
				.map(Self::AddX),
			Some("noop") => Ok(Self::Noop),
			_ => unreachable!(),
		}
	}
}

#[derive(Clone, Copy)]
struct Cpu {
	x: i64
}

impl Cpu {
	fn new() -> Self {
		Self { x: 1 }
	}

	fn exec(&mut self, instructions: &[Instruction]) -> CycleState {
		let snapshots = instructions.iter()
			.flat_map(|ins| {
				let vals = vec![*self; ins.num_cycles()];
				match ins {
					Instruction::Noop => (),
					Instruction::AddX(v) => self.x += v,
				};
				vals
			}).collect();
		CycleState { snapshots }
	}
}

struct CycleState {
	snapshots: Vec<Cpu>
}

impl CycleState {
	fn signal_strength(&self, n: usize) -> i64 {
		self.x(n - 1) * (n as i64)
	}

	fn x(&self, n: usize) -> i64 {
		self.snapshots[n].x
	}

	fn len(&self) -> usize {
		self.snapshots.len()
	}
}

#[derive(Copy, Clone)]
enum Pixel {
	Lit,
	Dark,
}

impl Default for Pixel {
	fn default() -> Self {
		Self::Dark
	}
}

impl fmt::Display for Pixel {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Self::Lit => f.write_char('#'),
			Self::Dark => f.write_char('.'),
		}
	}
}

struct CRT<const R: usize, const C: usize> {
	pixels: [[Pixel; C]; R],
}

impl<const R: usize, const C: usize> CRT<R, C> {
	fn new() -> Self {
		Self {
			pixels: [[Pixel::default(); C]; R],
		}
	}

	fn exec(&mut self, cycles: &CycleState) -> &Self {
		for i in 0..R {
			for j in 0..C {
				let x = cycles.x(i * C + j);
				let ipos = j as i64;
				if x - 1 <= ipos && ipos <= x + 1 {
					self.pixels[i][j] = Pixel::Lit;
				}
			}
		}
		self
	}
}

impl<const R: usize, const C: usize> fmt::Display for CRT<R, C> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		for i in 0..R {
			for j in 0..C {
				write!(f, "{}", self.pixels[i][j])?;
			}
			if i < R - 1 {
				writeln!(f)?;	
			}
		}
		Ok(())
	}
}

fn part1(cycles: &CycleState) -> i64 {
	(20..).step_by(40)
		.take_while(|i| *i < cycles.len())
		.map(|i| cycles.signal_strength(i))
		.sum()
}

fn part2(cycles: &CycleState) {
	println!("{}", CRT::<6, 40>::new().exec(cycles));
}

fn main() -> io::Result<()> {
	let inp = fs::read_to_string("./input.txt")?;
	let ins = inp.split('\n')
		.filter(|ln| !ln.trim().is_empty())
		.map(|ln| ln.parse::<Instruction>())
		.collect::<Result<Vec<_>, _>>()
		.map_err(|e| io::Error::new(ErrorKind::InvalidData, e))?;

	let cycles = Cpu::new().exec(&ins);
	println!("Part 1: {}", part1(&cycles));
	part2(&cycles);

	Ok(())
}
