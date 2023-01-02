use std::borrow::Borrow;
use std::collections::{HashSet, HashMap};
use std::fs;
use std::io::{self, ErrorKind};
use std::str::{self, FromStr};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Cell(u64);

impl Cell {
	fn can_climb<C: Borrow<Self>>(&self, other: C) -> bool {
		self.0 + 1 >= other.borrow().0
	}
}

impl From<char> for Cell {
	fn from(c: char) -> Self {
		match c {
			'S' => Self(0),
			'E' => Self::from('z'),
			_   => Self(((c as u8) - b'a') as u64)
		}
	}
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Coords {
	row: usize,
	col: usize,
}

impl Coords {
	fn new(row: usize, col: usize) -> Self {
		Self { row, col }
	}

	fn left(&self) -> Option<Coords> {
		self.col.checked_sub(1).map(|col| Self::new(self.row, col))
	}

	fn down(&self) -> Option<Coords> {
		self.row.checked_sub(1).map(|row| Self::new(row, self.col))
	}

	fn right(&self, limit: usize) -> Option<Coords> {
		match self.col + 1 {
			col if col == limit => None,
			col => Some(Self::new(self.row, col))
		}
	}

	fn up(&self, limit: usize) -> Option<Coords> {
		match self.row + 1 {
			row if row == limit => None,
			row => Some(Self::new(row, self.col))
		}
	}

	fn steps(&self, up_lim: usize, right_lim: usize) -> Vec<Coords> {
		[
			self.down(),
			self.left(),
			self.up(up_lim),
			self.right(right_lim),
		].into_iter()
			.flatten()
			.collect()
	}
}

#[derive(Debug, Clone)]
struct MapDistance(HashMap<Coords, usize>);

impl MapDistance {
	fn new() -> Self { Self(HashMap::new()) }

	fn get<C: Borrow<Coords>>(&self, p: C) -> Option<usize> {
		self.0.get(p.borrow()).copied()
	}

	fn set_path_len(&mut self, p: Coords, path_len: usize) {
		if self.0.get(&p).map(|old| *old > path_len).unwrap_or(true) {
			self.0.insert(p, path_len);
		}
	}
}

#[derive(Clone, Debug)]
struct Map {
	start: Coords,
	dst: Coords,
	map: Vec<Vec<Cell>>,
	paths: MapDistance,
}

impl Map {
	fn get<B: Borrow<Coords>>(&self, crd: B) -> Cell {
		self.map[crd.borrow().row][crd.borrow().col]
	}

	fn iter_coords(&self) -> impl Iterator<Item=Coords> + '_ {
		(0..self.map.len())
			.flat_map(|row| (0..self.map[0].len())
				.map(move |col| Coords::new(row, col)))
	}

	fn next_steps<C: Borrow<Coords> + Copy>(&self, cur: C) -> Vec<Coords> {
		let ulim = self.map.len();
		let rlim = self.map[0].len();
		cur.borrow().steps(ulim, rlim).into_iter()
			.filter(|p| self.get(p).can_climb(self.get(cur)))
			.collect()
	}

	fn rwalk(&mut self, path: &mut Vec<Coords>, seen: &mut HashSet<Coords>) {
		let cur = path.last().unwrap();
		self.paths.set_path_len(*cur, path.len());
		if *cur == self.start {
			return;
		}

		let mut possib = self.next_steps(cur);
		possib.retain(|p| !seen.contains(p));
		if possib.contains(&self.start) {
			possib.retain(|p| *p == self.start);
		}

		// For each possible next step, if we do not know a path for
		// it, or the known path is longer than our current one,
		// explore it
		for p in possib {
			if self.paths.get(p).map(|v| v > path.len() + 1)
				.unwrap_or(true)
			{
				path.push(p);
				seen.insert(p);
				self.rwalk(path, seen);
				path.pop();
				seen.remove(&p);
			}
		}
	}
}

impl FromStr for Map {
	type Err = io::Error;
	fn from_str(st: &str) -> Result<Self, Self::Err> {
		let mut start = None;
		let mut dst = None;
		let paths = MapDistance::new();

		let map = st.split('\n')
			.filter(|ln| !ln.trim().is_empty())
			.enumerate()
			.map(|(row, line)| line.chars()
				.enumerate()
				.inspect(|(col, chr)|
					match chr {
						'S' => start = Some(Coords::new(row, *col)),
						'E' => dst = Some(Coords::new(row, *col)),
						_ => (),
					})
				.map(|(_, chr)| Cell::from(chr))
				.collect())
			.collect::<Vec<Vec<_>>>();

		start.zip(dst)
			.map(|(start, dst)| Self { start, dst, map, paths })
			.ok_or_else(|| io::Error::new(ErrorKind::InvalidData, st))
	}
}

fn part1(map: &mut Map) -> usize {
	let mut path = vec![map.dst];
	let mut seen = HashSet::from([map.dst]);
	map.rwalk(&mut path, &mut seen);
	map.paths.get(map.start).unwrap() - 1
}

fn part2(map: &Map) -> usize {
	map.iter_coords()
		.filter(|c| map.get(c) == Cell(0))
		.filter_map(|c| map.paths.get(c))
		.min().unwrap() - 1
}

fn main() -> io::Result<()> {
	let inp = fs::read_to_string("./input.txt")?;
	let mut map = inp.parse::<Map>()?;

	println!("Part 1: {}", part1(&mut map));
	println!("Part 2: {}", part2(&map));

	Ok(())
}
