use std::cmp::{self, Ordering::*};
use std::fs;
use std::io::{self, ErrorKind};
use std::num::ParseIntError;
use std::str::FromStr;
use std::mem;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Coords {
	x: isize,
	y: isize,
}

impl Coords {
	fn manh_dist(&self, other: &Self) -> isize {
		(self.x.abs_diff(other.x) + self.y.abs_diff(other.y)) as isize
	}
}

// A horizontal line
#[derive(Clone, Copy, Debug)]
struct Line {
	start: isize,
	end: isize
}

impl Line {
	fn len(&self) -> isize {
		self.end - self.start + 1
	}

	fn overlap(&self, other: &Self) -> Option<Self> {
		let start = cmp::max(self.start, other.start);
		let end = cmp::min(self.end, other.end);
		match start <= end {
			false => None,
			true => Some(Line { start, end }),
		}
	}

	fn non_overlap(&self, segments: &mut Vec<Line>) {
		for i in (0..segments.len()).rev() {
			let cur = &segments[i];
			let overlap = match self.overlap(cur) {
				None => continue,
				Some(v) => v,
			};

			match (overlap.start.cmp(&cur.start),
				overlap.end.cmp(&cur.end))
			{
				(Equal, Equal) => { segments.swap_remove(i); },
				// segments[i]: |---------------------|
				// overlap:     |-----------|
				// result:                   |--------|
				(Equal, Less)  => {
					let mut seg = segments.get_mut(i).unwrap();
					seg.start = overlap.end + 1;
				},
				// segments[i]: |---------------------|
				// overlap:                  |--------|
				// result:      |-----------|
				(Greater, Equal) => {
					let mut seg = segments.get_mut(i).unwrap();
					seg.end = overlap.start - 1;
				},
				// segments[i]: |---------------------|
				// overlap:           |--------|
				// result:      |----|          |-----|
				(Greater, Less) => {
					let seg = segments.get_mut(i).unwrap();
					let end = mem::replace(&mut seg.end, overlap.start - 1);
					segments.push(Line { start: overlap.end + 1, end });
				},
				(..) => unreachable!(),
			}
		}
	}
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
struct Sensor {
	pos: Coords,
	radius: isize,
}

impl Sensor {
	/*
	 * Imagine a sensor with:
	 *    .pos = (x=5, y=5)
	 *    .radius = 4
	 * The sensor coverage looks like this:
	 *
	 *   0  1  2  3  4  5  6  7  8  9 10
	 * 0 .  .  .  .  .  .  .  .  .  .  .
	 * 1 .  .  .  .  .  #  .  .  .  .  .
	 * 2 .  .  .  .  #  #  #  .  .  .  .
	 * 3 .  .  .  #  #  #  #  #  .  .  .
	 * 4 .  .  #  #  #  #  #  #  #  .  .
	 * 5 .  #  #  #  #  x  #  #  #  #  .
	 * 6 .  .  #  #  #  #  #  #  #  .  .
	 * 7 .  .  .  #  #  #  #  #  .  .  .
	 * 8 .  .  .  .  #  #  #  .  .  .  .
	 * 9 .  .  .  .  .  #  .  .  .  .  .
	 *
	 * This function gives, for a given y, the sensor's
	 * coverage at that row.
	 * E.g.:
	 *  - For y = 7, the following line is returned:
	 *    start = 3, end = 7
	 *  - For y = 0, None is returned
	 */
	fn coverage_at(&self, y: isize) -> Option<Line> {
		let ydiff = y.abs_diff(self.pos.y);
		if ydiff >= self.radius as usize {
			return None;
		}
		let radius = self.radius - ydiff as isize;
		Some(Line {
			start: self.pos.x - radius,
			end:   self.pos.x + radius
		})
	}
}

fn find_x_dimensions<'a>(iter: impl Iterator<Item=&'a Sensor>) -> (isize, isize) {
	let mut min = isize::MAX;
	let mut max = isize::MIN;
	for s in iter {
		min = cmp::min(min, s.pos.x - s.radius);
		max = cmp::max(max, s.pos.x + s.radius);
	}
	(min, max)
}

fn part1<const ROW: isize>(sensors: &[Sensor]) -> isize {
	let (start, end) = find_x_dimensions(sensors.iter());
	let mut segments = vec![Line { start, end }];

	for sensor in sensors {
		if let Some(cv) = sensor.coverage_at(ROW) {
			cv.non_overlap(&mut segments);
		}
	}
	
	let avail = segments.iter()
		.map(|sg| sg.len())
		.sum::<isize>();

	(end - start) - avail
}

fn part2<const LIM: isize>(sensors: &[Sensor]) -> usize {
	let mut segments = Vec::with_capacity(4);

	for y in 0..=LIM {
		segments.push(Line { start: 0, end: LIM });
		for sensor in sensors.iter() {
			if let Some(cv) = sensor.coverage_at(y) {
				cv.non_overlap(&mut segments);
				if segments.is_empty() {
					break;
				}
			}
		}

		if !segments.is_empty() {
			assert_eq!(segments.len(), 1);
			return (segments[0].start * 4000000 + y) as usize;
		}

		segments.clear();
	}

	unreachable!()
}

impl FromStr for Sensor {
	type Err = ParseIntError;
	fn from_str(st: &str) -> Result<Self, Self::Err> {
		let mut fields = st.split(' ');
		assert_eq!(fields.next(), Some("Sensor"));
		assert_eq!(fields.next(), Some("at"));
		
		let x = fields.next().unwrap();
		assert!(x.starts_with("x="));
		let x = x[2..x.len() - 1].parse::<isize>()?;

		let y = fields.next().unwrap();
		assert!(y.starts_with("y="));
		let y = y[2..y.len() - 1].parse::<isize>()?;

		let pos = Coords { x, y };

		assert_eq!(fields.next(), Some("closest"));
		assert_eq!(fields.next(), Some("beacon"));
		assert_eq!(fields.next(), Some("is"));
		assert_eq!(fields.next(), Some("at"));

		let x = fields.next().unwrap();
		assert!(x.starts_with("x="));
		let x = x[2..x.len() - 1].parse::<isize>()?;

		let y = fields.next().unwrap();
		assert!(y.starts_with("y="));
		let y = y[2..].parse::<isize>()?;

		let beacon = Coords { x, y };
		let radius = pos.manh_dist(&beacon);

		Ok(Self { pos, radius })
	}
}

fn main() -> io::Result<()> {
	let inp = fs::read_to_string("./input.txt")?;

	let mut sensors = inp.split('\n')
		.filter(|ln| !ln.trim().is_empty())
		.map(Sensor::from_str)
		.collect::<Result<Vec<_>, _>>()
		.map_err(|e| io::Error::new(ErrorKind::InvalidData, e))?;

	sensors.sort_by_key(|s| cmp::Reverse(s.radius));

	println!("Part 1: {}", part1::<2000000>(&sensors));
	println!("Part 2: {}", part2::<4000000>(&sensors));

	Ok(())
}
