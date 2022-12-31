use std::fs;
use std::io;
use std::iter;
use std::str;
use std::collections::HashMap;

const TOTAL_SPACE: usize = 70000000;
const NEED_UNUSED_SPACE: usize = 30000000;

#[derive(Debug)]
enum Node<'a> {
	File(&'a str, usize),
	Dir(&'a str, HashMap<&'a str, Node<'a>>)
}

impl<'a> Node<'a> {
	fn new_file(name: &'a str, sz: usize) -> Self {
		Self::File(name, sz)
	}

	fn new_dir(name: &'a str) -> Self {
		Self::Dir(name, HashMap::new())
	}

	fn name(&self) -> &'a str {
		match self {
			Self::File(name, _) |
			Self::Dir(name, _) => name,
		}
	}

	fn size(&self) -> usize {
		match self {
			Self::File(_, sz) => *sz,
			Self::Dir(_, h) => h.values()
				.map(|d| d.size()).sum(),
		}
	}

	fn is_dir(&self) -> bool {
		matches!(self, Self::Dir(_, _))
	}

	fn children_dir(&self) -> impl Iterator<Item=&Node<'a>> {
		match self {
			Self::Dir(_, h) => h.values().filter(|c| c.is_dir()),
			_ => unreachable!(),
		}
	}

	// Adds a node at the specified path
	fn add_subnode_path(&mut self, path: &[&str], node: Node<'a>) {
		match self {
			Self::Dir(name, h) => {
				assert!(&path[0] == name);
				match path.len() {
					1 => { h.insert(node.name(), node); },
					_ => { h.get_mut(path[1])
						.unwrap().add_subnode_path(&path[1..], node); },
				}
			},
			_ => unreachable!(),
		}
	}

	fn part1(&self) -> usize {
		self.children_dir()
			.map(|ch| {
				let size = ch.size();
				ch.part1() + (size <= 100000)
					.then_some(size).unwrap_or(0)
			})
			.sum()
	}

	fn part2(&self, needed: usize) -> Option<usize> {
		self.children_dir()
			.filter_map(|c| c.part2(needed))
			.chain(iter::once(self.size()))
			.filter(|sz| *sz >= needed)
			.min()
	}
}

#[derive(Debug)]
enum Command<'a> {
	Cd(&'a str),
	Ls(Vec<Node<'a>>),
}

impl<'a> Command<'a> {
	fn parse_command(inp: &'a [&'a str]) -> (Self, &'a [&'a str]) {
		let mut cmd_str = inp[0].split(' ');
		assert!(cmd_str.next() == Some("$"));

		let (linenum, cmd) = match cmd_str.next() {
			Some("cd") => (1, Command::Cd(cmd_str.next().unwrap())),
			Some("ls") => {
				let nodes = inp[1..].iter()
					.take_while(|ln| !ln.starts_with('$'))
					.map(|ln| {
						let mut fields = ln.split(' ');
						match (fields.next(), fields.next()) {
							(Some("dir"), Some(name)) => Node::new_dir(name),
							(Some(size), Some(name)) => Node::new_file(name,
								size.parse::<usize>().unwrap()),
							(..) => unreachable!(),
						}
					})
					.collect::<Vec<_>>();
				(nodes.len() + 1, Command::Ls(nodes))
			}
			_ => unreachable!(),
		};

		(cmd, &inp[linenum..])
	}
}

struct Commands<'a>(Vec<Command<'a>>);

impl<'a> From<&'a [&'a str]> for Commands<'a> {
	fn from(mut lines: &'a [&'a str]) -> Self {
		let mut cmds = Vec::new();
		while !lines.is_empty() {
			let res = Command::parse_command(lines);
			cmds.push(res.0);
			lines = res.1;
		}
		Self(cmds)
	}
}

impl<'a> Commands<'a> {
	fn get_tree(self) -> Node<'a> {
		let mut root = Node::new_dir("/");
		let mut dirstack = Vec::with_capacity(10);

		for cmd in self.0.into_iter() {
			match cmd {
				Command::Cd(name) => match name {
					"/" => {
						dirstack.clear();
						dirstack.push("/");
					},
					".." => {
						dirstack.pop().unwrap();
					},
					d => dirstack.push(d),
				},
				Command::Ls(files) => {
					for file in files.into_iter() {
						root.add_subnode_path(&dirstack, file);
					}
				}
			}
		}

		root
	}
}

fn main() -> io::Result<()> {
	let inp = fs::read_to_string("./input.txt")?;

	let lines = inp.split('\n')
		.filter(|ln| !ln.trim().is_empty())
		.collect::<Vec<_>>();

	let tree = Commands::from(&lines[..]).get_tree();
	println!("{}", tree.part1());

	let unused_space = TOTAL_SPACE - tree.size();
	let must_free = NEED_UNUSED_SPACE - unused_space;
	println!("{}", tree.part2(must_free).unwrap());

	Ok(())
}
