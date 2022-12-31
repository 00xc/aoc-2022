use std::io;
use std::fmt;
use std::fs;
use std::str;
use std::ops::{Deref, DerefMut};

fn parse_line_crates(st: &str) -> Vec<Option<Crate>> {
	let mut fields = Vec::new();
	let mut pos: usize = 0;
	while pos < st.len() {
		let field = &st[pos..pos + 3];
		pos += 3;
		fields.push(field.parse::<Crate>().ok());
		pos += 1;
	}
	fields
}

#[derive(Debug, Clone)]
struct Drawing {
	columns: Vec<Column>
}

impl Drawing {
	fn exec(&mut self, movs: &[Movement]) -> &Self {
		for mov in movs.iter() {
			for _ in 0..mov.amnt {
				let cr = self.columns[mov.from]
					.pop().unwrap();
				self.columns[mov.to].push(cr);
			}
		}
		self
	}

	fn exec_9001(&mut self, movs: &[Movement]) -> &Self {
		for mov in movs.iter() {
			let cr = self.columns[mov.from]
				.pop_multi(mov.amnt).unwrap();
			self.columns[mov.to].push_multi(cr);
		}
		self
	}

	fn fmt(&self) -> String {
		let depth = self.columns.iter()
			.map(|col| col.len())
			.max().unwrap();

		(0..depth)
			.map(|row| self.columns.iter()
				.map(|col| col.get(row)
					.map(|cr| cr.to_string())
					.unwrap_or_else(|| "[_]".to_string()))
				.collect::<Vec<_>>()
				.join(" "))
			.rev()
			.collect::<Vec<_>>()
			.join("\n")
	}

	fn msg(&self) -> String {
		self.columns.iter()
			.filter_map(|col| col.last())
			.map(|cr| String::from(cr.name))
			.collect::<Vec<_>>()
			.join("")
	}
}

impl str::FromStr for Drawing {
	type Err = io::Error;
	fn from_str(s: &str) -> Result<Self, Self::Err> {
		// Parse out all crates in each row, except last one
		let mut rows = s.split('\n')
			.filter(|ln| !ln.trim().is_empty())
			.collect::<Vec<_>>();
		rows.pop();
		let rows = rows.into_iter()
			.map(parse_line_crates)
			.collect::<Vec<_>>();

		// Prepare columns
		let width = rows[0].len();
		let mut columns = (0..width)
			.map(|_| Column::new())
			.collect::<Vec<_>>();

		// Transpose rows into columns
		for row in rows.into_iter() {
			for (i, cr) in row.into_iter().enumerate() {
				if let Some(v) = cr {
					columns[i].push(v);
				}
			}
		}

		for col in columns.iter_mut() {
			col.reverse();
		}

		Ok(Self { columns })
	}
}

#[derive(Debug, Clone)]
struct Column(Vec<Crate>);

impl Deref for Column {
	type Target = Vec<Crate>;
	fn deref(&self) -> &Self::Target { &self.0 }
}

impl DerefMut for Column {
	fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 }
}

impl Column {
	fn new() -> Self {
		Self(<Self as Deref>::Target::new())
	}

	fn pop_multi(&mut self, n: usize) -> Option<Vec<Crate>> {
		let mut out = (0..n).map(|_| self.pop())
			.collect::<Option<Vec<_>>>()?;
		out.reverse();
		Some(out)
	}

	fn push_multi(&mut self, v: Vec<Crate>) {
		for e in v.into_iter() {
			self.push(e);
		}
	}
}

#[derive(Debug, Copy, Clone)]
struct Crate {
	name: char,
}

impl str::FromStr for Crate {
	type Err = String;
	fn from_str(s: &str) -> Result<Self, Self::Err> {
		if s.trim().is_empty() {
			return Err("Invalid empty crate".to_string())
		}
		let mut chrs = s.chars();
		assert!(chrs.next().unwrap() == '[');
		let name = chrs.next().unwrap();
		assert!(chrs.next().unwrap() == ']');
		Ok(Self { name })
	}
}

impl fmt::Display for Crate {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "[{}]", self.name)
	}
}

#[derive(Debug, Clone, Copy)]
struct Movement {
	amnt: usize,
	from: usize,
	to: usize,
}

impl str::FromStr for Movement {
	type Err = String;
	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let mut parts = s.split(' ');
		assert!(parts.next().unwrap() == "move");
		let amnt = parts.next().unwrap().parse::<usize>().unwrap();
		assert!(parts.next().unwrap() == "from");
		let from = parts.next().unwrap().parse::<usize>().unwrap() - 1;
		assert!(parts.next().unwrap() == "to");
		let to = parts.next().unwrap().parse::<usize>().unwrap() - 1;
		Ok(Self { amnt, from, to })
	}
}

fn main() -> io::Result<()> {
	let inp = fs::read_to_string("./input.txt")?;

	let parts = inp.split("\n\n").collect::<Vec<_>>();
	let drawing = parts[0].parse::<Drawing>().unwrap();
	let moves = parts[1]
		.split('\n')
		.filter(|line| !line.trim().is_empty())
		.map(|line| line.parse::<Movement>())
		.collect::<Result<Vec<_>, _>>().unwrap();

	println!("{}\n----", drawing.fmt());

	let part1 = drawing.clone().exec(&moves).msg();
	println!("Part 1: {}", part1);

	let part2 = drawing.clone().exec_9001(&moves).msg();
	println!("Part 2: {}", part2);

	Ok(())
}
