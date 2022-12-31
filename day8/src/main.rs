use std::fs;
use std::io::{self, ErrorKind};
use std::str;
use std::num::ParseIntError;

type Tree = u8;

#[derive(Debug)]
struct Forest {
	rows: Vec<Vec<Tree>>
}

impl Forest {
	fn width(&self) -> usize {
		self.rows[0].len()
	}

	fn height(&self) -> usize {
		self.rows.len()
	}

	fn tree_is_left_edge(&self, _row: usize, col: usize) -> bool {
		col == 0
	}

	fn tree_is_right_edge(&self, _row: usize, col: usize) -> bool {
		col == self.width() - 1
	}

	fn tree_is_up_edge(&self, row: usize, _col: usize) -> bool {
		row == 0
	}

	fn tree_is_down_edge(&self, row: usize, _col: usize) -> bool {
		row == self.height() - 1
	}

	fn tree_is_edge(&self, row: usize, col: usize) -> bool {
		self.tree_is_left_edge(row, col) ||
			self.tree_is_up_edge(row, col) ||
			self.tree_is_down_edge(row, col) ||
			self.tree_is_right_edge(row, col)			
	}

	fn column(&self, col: usize) -> impl Iterator<Item=Tree> + '_ {
		self.rows.iter()
			.map(move |row| row[col])
	}

	fn tree(&self, row: usize, col: usize) -> Tree {
		self.rows[row][col]
	}

	fn tree_visible(&self, row: usize, col: usize) -> bool{
		if self.tree_is_edge(row, col) {
			return true;
		}

		let tree = self.tree(row, col);
		let rowv = &self.rows[row];

		if rowv[..col].iter().all(|t| *t < tree) {
			return true;
		}

		if rowv[col + 1..].iter().all(|t| *t < tree) {
			return true;
		}

		let column = self.column(col).collect::<Vec<_>>();

		if column[..row].iter().all(|t| *t < tree) {
			return true;
		}

		if column[row + 1..].iter().all(|t| *t < tree) {
			return true;
		}

		false
	}

	fn tree_score(&self, row: usize, col: usize) -> usize {
		let tree = self.tree(row, col);
		let rowv = &self.rows[row];

		let mut left = rowv[..col].iter()
			.rev()
			.take_while(|t| **t < tree)
			.count();
		left += !self.tree_is_left_edge(row, col - left) as usize;

		let mut right = rowv[col + 1..].iter()
			.take_while(|t| **t < tree)
			.count();
		right += !self.tree_is_right_edge(row, col + right) as usize;

		let column = self.column(col).collect::<Vec<_>>();

		let mut up = column[..row].iter()
			.rev()
			.take_while(|t| **t < tree)
			.count();
		up += !self.tree_is_up_edge(row - up, col) as usize;

		let mut down = column[row + 1..].iter()
			.take_while(|t| **t < tree)
			.count();
		down += !self.tree_is_down_edge(row + down, col) as usize;

		left * right * down * up

	}

	fn iter_coords(&self) -> impl Iterator<Item=(usize, usize)> + '_ {
		(0..self.width())
			.flat_map(move |i| (0..self.height())
				.map(move |j| (i, j)))
	}

	fn part1(&self) -> usize {
		self.iter_coords()
			.filter(|(i, j)| self.tree_visible(*i, *j))
			.count()
	}

	fn part2(&self) -> usize {
		self.iter_coords()
			.map(|(i, j)| self.tree_score(i, j))
			.max().unwrap()
	}
}

impl str::FromStr for Forest {
	type Err = ParseIntError;
	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let v = s.split('\n')
			.filter(|line| !line.trim().is_empty())
			.map(|line| line.split_terminator("")
				.skip(1)
				.map(|c| c.parse::<Tree>())
				.collect::<Result<Vec<_>, _>>())
			.collect::<Result<Vec<_>, _>>()?;
		Ok(Self { rows: v })
	}
}

fn main() -> io::Result<()> {
	let inp = fs::read_to_string("./input.txt")?;
	let forest = inp.parse::<Forest>()
		.map_err(|e| io::Error::new(ErrorKind::InvalidData, e))?;

	println!("Part 1: {}", forest.part1());
	println!("Part 2: {}", forest.part2());
	
	Ok(())
}
