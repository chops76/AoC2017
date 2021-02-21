use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;

type Input = Vec<Vec<char>>;

fn parse_input(path: &str) -> Input {
	let f = File::open(path).unwrap();
	BufReader::new(f).lines().flatten().map(|s| s.chars().collect::<Vec<char>>()).collect()
}

fn part1(grid: &Input) -> String {
	let start_x = grid[0].iter().position(|&c| c == '|').unwrap();
	let mut pos: (i32, i32)  = (0, start_x as i32);
	let mut dir: (i32, i32) = (1, 0);
	let mut letters = Vec::new();
	while grid[pos.0 as usize][pos.1 as usize] != ' ' {
		pos.0 += dir.0;
		pos.1 += dir.1;
		if grid[pos.0 as usize][pos.1 as usize].is_alphabetic() {
			letters.push(grid[pos.0 as usize][pos.1 as usize]);
		}
		if grid[pos.0 as usize][pos.1 as usize] == '+' {
			if dir != (1, 0) && grid[pos.0 as usize-1][pos.1 as usize] != ' ' {
				dir = (-1, 0);
			} else if dir != (-1, 0) && grid[pos.0 as usize+1][pos.1 as usize] != ' ' {
				dir = (1, 0);
			} else if dir != (0, 1) && grid[pos.0 as usize][pos.1 as usize-1] != ' ' {
				dir = (0, -1);
			} else if dir != (0, -1) && grid[pos.0 as usize][pos.1 as usize+1] != ' ' {
				dir = (0, 1);
			}
		}
	}

	letters.iter().collect()
}

fn part2(grid: &Input) -> usize {
	let start_x = grid[0].iter().position(|&c| c == '|').unwrap();
	let mut pos: (i32, i32)  = (0, start_x as i32);
	let mut dir: (i32, i32) = (1, 0);
	let mut steps = 0;
	while grid[pos.0 as usize][pos.1 as usize] != ' ' {
		steps += 1;
		pos.0 += dir.0;
		pos.1 += dir.1;
		if grid[pos.0 as usize][pos.1 as usize] == '+' {
			if dir != (1, 0) && grid[pos.0 as usize-1][pos.1 as usize] != ' ' {
				dir = (-1, 0);
			} else if dir != (-1, 0) && grid[pos.0 as usize+1][pos.1 as usize] != ' ' {
				dir = (1, 0);
			} else if dir != (0, 1) && grid[pos.0 as usize][pos.1 as usize-1] != ' ' {
				dir = (0, -1);
			} else if dir != (0, -1) && grid[pos.0 as usize][pos.1 as usize+1] != ' ' {
				dir = (0, 1);
			}
		}
	}

	steps
}

pub fn main() {
	let grid = parse_input("./input/day19/input.txt");

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