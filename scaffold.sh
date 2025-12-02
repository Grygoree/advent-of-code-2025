#!/usr/bin/env bash
set -e

DAYS=31

for i in $(seq -w 1 $DAYS); do
	name="day$i"
	echo "Creating $name..."
	cargo new --bin --vcs none "$name"

	touch "$name/input.txt"
	cat > "$name/src/main.rs" << 'MAIN'

fn main() {
    let input = include_str!("input.txt");

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

pub fn part1(input: &str) -> i64 {
    // TODO
    0
}

pub fn part2(input: &str) -> i64 {
    // TODO
    0
}
MAIN
done

echo "Scaffolding complete."
