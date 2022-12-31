use std::io;
use std::fs;
use std::str;
use std::collections::HashSet;
use std::borrow::Borrow;

#[derive(Copy, Clone, Debug)]
enum Movement {
	Right,
	Left,
	Up,
	Down,
	None
}

impl Movement {
	fn flat_from_str(s: &str) -> Vec<Movement> {
		let mut fields = s.split(' ');
		let mov = match fields.next() {
			Some("R") => Self::Right,
			Some("L") => Self::Left,
			Some("U") => Self::Up,
			Some("D") => Self::Down,
			_ => unreachable!(),
		};
		let amount = fields.next().unwrap()
			.parse::<usize>().unwrap();
		vec![mov; amount]
	}
}

#[derive(Copy, Clone, Debug)]
struct Knot {
	row: i64,
	col: i64
}

impl Knot {
	fn new() -> Self {
		Self { row: 0, col: 0 }
	}

	fn pos(&self) -> (i64, i64) {
		(self.row, self.col)
	}

	fn mov<M: Borrow<Movement>>(&mut self, mov: M) {
		match mov.borrow() {
			Movement::Right => self.col += 1,
			Movement::Left  => self.col -= 1,
			Movement::Up    => self.row += 1,
			Movement::Down  => self.row -= 1,
			Movement::None  => (),
		}
	}

	fn touching(&self, other: &Self) -> bool {
		self.row.abs_diff(other.row) < 2 &&
			self.col.abs_diff(other.col) < 2
	}

	fn catchup_horizontal(&self, diff: i64) -> Movement {
		match diff {
			v if v > 0 => Movement::Right,
			v if v < 0 => Movement::Left,
			_ => Movement::None,
		}
	}

	fn catchup_vertical(&self, diff: i64) -> Movement {
		match diff {
			v if v > 0 => Movement::Up,
			v if v < 0 => Movement::Down,
			_ => Movement::None,
		}
	}

	fn catchup(&self, other: &Self) -> [Movement; 2] {
		match self.touching(other) {
			true => [Movement::None; 2],
			false => [
				self.catchup_horizontal(other.col - self.col),
				self.catchup_vertical(other.row - self.row)
			]
		}
	}
}

fn simulate<const N: usize>(movs: &[Movement]) -> usize {
	let mut knots = [Knot::new(); N];
	let mut visited = HashSet::new();

	for mov in movs {
		knots[0].mov(mov);
		for i in 1..N {
			for m in knots[i].catchup(&knots[i - 1]) {
				knots[i].mov(m);
			}
		}
		visited.insert(knots[N - 1].pos());
	}

	visited.len()
}

fn main() -> io::Result<()> {
	let inp = fs::read_to_string("./input.txt")?;
	let movs = inp.split('\n')
		.filter(|ln| !ln.trim().is_empty())
		.flat_map(Movement::flat_from_str)
		.collect::<Vec<_>>();

	println!("Part 1: {}", simulate::<2>(&movs));
	println!("Part 2: {}", simulate::<10>(&movs));

	Ok(())
}
