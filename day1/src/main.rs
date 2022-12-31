use std::fs;
use std::io;

fn main() -> io::Result<()> {
    let inp = fs::read_to_string("./input.txt")?;

    let elves = inp.split("\n\n")
        .map(|e| e.split('\n')
            .filter(|c| !c.is_empty())
            .map(|c| c.parse::<u64>())
            .collect::<Result<Vec<_>, _>>())
        .collect::<Result<Vec<_>, _>>()
        .unwrap();

    let mut calories = elves.iter()
        .map(|c| c.iter().sum::<u64>())
        .collect::<Vec<_>>();

    println!("Part 1: {}", calories.iter().max().unwrap());

    calories.sort();
    let top = calories.iter()
        .rev()
        .take(3)
        .sum::<u64>();

    println!("Part 2: {}", top);

    Ok(())
}
