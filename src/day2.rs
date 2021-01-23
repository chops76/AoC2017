use std::fs::File;
use std::time::Instant;
use std::io::{BufRead, BufReader};
use std::cmp;

type Input = Vec<Vec<i32>>;

fn parse_line(s: &str) -> Vec<i32>{
	s.split_whitespace().map(|v| v.parse().unwrap()).collect()
}

fn parse_input(path: &str) -> Input {
	let f = File::open(path).unwrap();
	BufReader::new(f).lines().flatten().map(|s| parse_line(&s)).collect()
}

fn part1(vals: &Input) -> i32 {
	vals.iter()
		.map(|v| v.iter().max().unwrap() - v.iter().min().unwrap())
		.sum()
}

fn evenly_divis(nums: &Vec<i32>) -> i32 {
	for i in 0..nums.len() - 1 {
		for j in i+1..nums.len() {
			let a = cmp::max(nums[i], nums[j]);
			let b = cmp::min(nums[i], nums[j]);
			if a % b == 0 {
				return a / b;
			}
		}
	}
	0
}

fn part2(vals: &Input) -> i32 {
	vals.iter()
		.map(|v| evenly_divis(v))
		.sum()
}

pub fn main() {
	let vals = parse_input("./input/day2/input.txt");

	let p1_timer = Instant::now();
    let p1_result = part1(&vals);
    let p1_time = p1_timer.elapsed();
	println!("Part 1: {}", p1_result);
	println!("Part 1 Time: {:?}", p1_time);

	let p2_timer = Instant::now();
    let p2_result = part2(&vals);
    let p2_time = p2_timer.elapsed();
	println!("Part 2: {}", p2_result);
	println!("Part 2 Time: {:?}", p2_time);
}
