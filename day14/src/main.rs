use std::error::Error;
use std::fs;
use std::io::{self, ErrorKind};
use std::str::FromStr;
use std::marker::{Send, Sync};

fn min_max<T: Ord>(a: T, b: T) -> (T, T) {
	match a < b {
		true  => (a, b),
		false => (b, a),
	}
}

fn invalid_data<E>(e: E) -> io::Error
where
	E: Into<Box<dyn Error + Send + Sync>>
{
	io::Error::new(ErrorKind::InvalidData, e)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Coords {
	x: usize,
	y: usize
}

impl FromStr for Coords {
	type Err = io::Error;
	fn from_str(st: &str) -> Result<Self, Self::Err> {
		let mut parts = st.split(',');
		let (x, y) = parts.next().zip(parts.next())
			.ok_or_else(|| invalid_data(st))?;
		let x = x.parse::<usize>().map_err(invalid_data)?;
		let y = y.parse::<usize>().map_err(invalid_data)?;
		Ok(Self { x, y })
	}
}

type RockPath = Vec<Coords>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Cell {
	Rock,
	Air,
	Sand,
}

#[derive(Debug, Clone)]
struct Map {
	map: Vec<Vec<Cell>>,
	sand: Coords,
}

impl Map {
	fn new(width: usize, height: usize, sand: Coords) -> Self {
		Self {
			sand,
			map: (0..height)
				.map(|_| (0..width)
					.map(|_| Cell::Air)
					.collect())
				.collect()
		}   
	}

	fn set_rock_paths(&mut self, paths: &[RockPath]) {
		for path in paths {
			for i in 1..path.len() {
				let a = &path[i - 1];
				let b = &path[i];
				let (min_x, max_x) = min_max(a.x, b.x);
				let (min_y, max_y) = min_max(a.y, b.y);
				match (a.x == b.x, a.y == b.y) {
					(true, false) => (min_y..=max_y)
						.for_each(|y| self.map[y][a.x] = Cell::Rock),
					(false, true) => (min_x..=max_x)
						.for_each(|x| self.map[a.y][x] = Cell::Rock),
					(..) => unreachable!(),
				}
			}
		}
	}

	fn draw(&self, pos: &Coords)  {
		for (y, row) in self.map.iter().enumerate() {
			for (x, c) in row.iter().enumerate() {
				if x < 490 { continue; }
				match c {
					_ if x == pos.x && y == pos.y => print!("x"),
					Cell::Air  => print!("."),
					Cell::Sand => print!("o"),
					Cell::Rock => print!("#"),
				}
			}
			println!();
		}
	}

	fn part1(&mut self) -> usize {
		let mut num = 0;
		let mut sand = self.sand;

		while sand.y < self.map.len() - 1 {
			match (self.map[sand.y + 1].get(sand.x - 1),
				self.map[sand.y + 1].get(sand.x),
				self.map[sand.y + 1].get(sand.x + 1))
			{
				(_, Some(Cell::Air), _) =>   sand.y += 1,
				(Some(Cell::Air), _, _) => { sand.x -= 1; sand.y += 1; },
				(_, _, Some(Cell::Air)) => { sand.x += 1; sand.y += 1; },
				(..) => {
					self.map[sand.y][sand.x] = Cell::Sand;
					sand = self.sand;
					num += 1;
				}
			}
		}
		num
	}

	fn reset(&mut self) {
		self.map.iter_mut()
			.for_each(|row| row.iter_mut()
				.filter(|c| **c == Cell::Sand)
				.for_each(|c| *c = Cell::Air));
	}

	fn part2(&mut self) -> usize {
		let mut num = 0;
		let mut sand = self.sand;

		loop {
			match (self.map[sand.y + 1].get(sand.x - 1),
				self.map[sand.y + 1].get(sand.x),
				self.map[sand.y + 1].get(sand.x + 1))
			{
				(_, Some(Cell::Air), _) =>   sand.y += 1,
				(Some(Cell::Air), _, _) => { sand.x -= 1; sand.y += 1; },
				(_, _, Some(Cell::Air)) => { sand.x += 1; sand.y += 1; },
				(..) => {
					self.map[sand.y][sand.x] = Cell::Sand;
					num += 1;
					if sand == self.sand {
						break;
					}
					sand = self.sand;
				}
			}
		}

		num
	}
}

fn get_map_size(paths: &[RockPath]) -> (usize, usize) {
	let mut width: usize = 0;
	let mut height: usize = 0;
	for path in paths {
		for c in path {
			if c.x > width { width = c.x; }
			if c.y > height { height = c.y; }
		}
	}
	(width * 2, height + 3)
}

fn main() -> io::Result<()> {
	let inp = fs::read_to_string("./input.txt")?;
	let paths = inp.split('\n')
		.filter(|ln| !ln.trim().is_empty())
		.map(|ln| ln.split(" -> ").map(Coords::from_str).collect())
		.collect::<io::Result<Vec<_>>>()?;

	let (width, height) = get_map_size(&paths);
	let mut map = Map::new(width, height, Coords { x: 500, y: 0 });

	map.set_rock_paths(&paths);
	println!("Part 1: {}", map.part1());

	map.reset();

	let floor = vec![
		Coords { x: 0, y: height - 1 },
		Coords { x: width - 1, y: height - 1 }
	];
	map.set_rock_paths(&[floor]);
	println!("Part 2: {}", map.part2());

	Ok(())
}
