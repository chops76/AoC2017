use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;
use std::collections::HashMap;

#[derive(Clone)]
#[derive(Copy)]
enum State {
	CLEAN,
	INFECTED,
	WEAKENED,
	FLAGGED
}

fn parse_line(i: i32, s: &str, hm: &mut HashMap<(i32, i32), bool>) {
	for (j, c) in s.chars().enumerate() {
		hm.insert((j as i32, i), c == '#');
	}
}

fn parse_input(path: &str) -> HashMap<(i32, i32), bool> {
	let mut hm = HashMap::new();
	let f = File::open(path).unwrap();
	for (i, line) in BufReader::new(f).lines().flatten().enumerate() {
		parse_line(i as i32, &line, &mut hm);
	}
	hm
}

fn part1(hm: &HashMap<(i32, i32), bool>) -> usize {
	let heading: Vec<(i32, i32)>= vec![(0, -1), (1, 0), (0, 1), (-1, 0)];
	let mut direction = 0;
	let mut pos = (12, 12);
	let mut grid = hm.clone();

	let mut infected = 0;

	for _ in 0..10000 {
		let cur_pos = pos;
		let inf = if !grid.contains_key(&cur_pos) { false } else { grid[&cur_pos] };
		if inf {
			direction = (direction + 1) % 4;
		} else {
			direction = (direction + 3) % 4;
		}
		grid.insert(cur_pos, !inf);
		if !inf {
			infected += 1;
		}
		pos = (pos.0 + heading[direction].0, pos.1 + heading[direction].1);
	}

	infected
}

fn part2(hm: &HashMap<(i32, i32), bool>) -> usize {
	let heading: Vec<(i32, i32)>= vec![(0, -1), (1, 0), (0, 1), (-1, 0)];
	let mut direction = 0;
	let mut pos = (12, 12);
	let mut grid = HashMap::new();

	for (pos, state) in hm {
		grid.insert(*pos, if *state {State::INFECTED} else {State::CLEAN});
	}

	let mut infected = 0;

	for _ in 0..10000000 {
		let cur_pos = pos;
		let inf = if !grid.contains_key(&cur_pos) { State::CLEAN } else { grid[&cur_pos] };
		match inf {
			State::CLEAN => { 
				direction = (direction + 3) % 4;
				grid.insert(cur_pos, State::WEAKENED); 
			},
			State::FLAGGED => { 
				direction = (direction + 2) % 4;
				grid.insert(cur_pos, State::CLEAN);
			},
			State::INFECTED => { 
				direction = (direction + 1) % 4;
				grid.insert(cur_pos, State::FLAGGED);
			},
			State::WEAKENED => {
				infected += 1;
				grid.insert(cur_pos, State::INFECTED);
			}
		}
		pos = (pos.0 + heading[direction].0, pos.1 + heading[direction].1);
	}

	infected
}

pub fn main() {
	let grid = parse_input("./input/day22/input.txt");
	
	let p1_timer = Instant::now();
    let p1_result = part1(&grid);
    let p1_time = p1_timer.elapsed();
	println!("Part 1: {}", p1_result);
	println!("Part 1 Time: {:?}", p1_time);

	let p2_timer = Instant::now();
    let p2_result = part2(&grid);
    let p2_time = p2_timer.elapsed();
	println!("Part 2: {}", p2_result);
	println!("Part 2 Time: {:?}", p2_time);
}