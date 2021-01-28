use std::io::Read;
use std::fs::File;
use std::time::Instant;
use std::collections::HashSet;

fn parse_input(path: &str) -> Vec<u32> {
	let mut fstr = String::new();

	File::open(path).unwrap().read_to_string(&mut fstr);
	fstr.split_whitespace().map(|s| s.parse().unwrap()).collect()
}

pub fn part1(vals_p: &Vec<u32>) -> usize {
	let mut count = 0;
	let mut vals = vals_p.clone();
	let mut hs = HashSet::new();

	while !hs.contains(&vals) {
		count += 1;
		hs.insert(vals.clone());
		let max_idx = vals.iter().enumerate().max_by_key(|(i, &k)| k*100+(99-*i as u32)).map(|(i, _)| i).unwrap();
		let max_val = vals[max_idx];
		vals[max_idx] = 0;
		for i in 1..=max_val {
			vals[(max_idx+i as usize)%vals_p.len()] += 1;
		}
	}
	count
}

pub fn part2(vals_p: &Vec<u32>) -> usize {
	let mut count = 0;
	let mut vals = vals_p.clone();
	let mut hs = HashSet::new();
	let mut cycling = false;

	loop {
		if hs.contains(&vals) {
			if cycling {
				return count;
			}
			hs = HashSet::new();
			cycling = true;
			count = 0;
		}
		count += 1;
		hs.insert(vals.clone());
		let max_idx = vals.iter().enumerate().max_by_key(|(i, &k)| k*100+(99-*i as u32)).map(|(i, _)| i).unwrap();
		let max_val = vals[max_idx];
		vals[max_idx] = 0;
		for i in 1..=max_val {
			vals[(max_idx+i as usize)%vals_p.len()] += 1;
		}
	}
	count
}

pub fn main() {
	let vals = parse_input("./input/day6/input.txt");

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