use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;
use std::collections::HashMap;

type Input = HashMap<Vec<Vec<bool>>, Vec<Vec<bool>>>;

fn rotate(array: &Vec<Vec<bool>>) -> Vec<Vec<bool>> {
	let mut a = array.clone();
	let n = a.len();
	for i in 0..n-1 {
		for j in i..n-1-i {
			let temp = a[i][j];
			a[i][j] = a[n-1-j][i];
			a[n-1-j][i] = a[n-1-i][n-1-j];
			a[n-1-i][n-1-j] = a[j][n-1-i];
			a[j][n-1-i] = temp;
		}
	}

	a
}

fn flip(array: &Vec<Vec<bool>>) -> Vec<Vec<bool>> {
	let mut a = array.clone();
	let n = a.len();
	for i in 0..n {
		for j in 0..n/2 {
			let temp = a[i][j];
			a[i][j] = a[i][n-1-j];
			a[i][n-1-j] = temp;
		}
	}

	a
}

fn find_match(array: &Vec<Vec<bool>>, rules: &Input) -> Vec<Vec<bool>> {
	let mut a = array.clone();
	for _ in 0..4 {
		if rules.contains_key(&a) {
			return rules[&a].clone();
		}
		a = rotate(&a);
	}
	a = flip(&a);
	for _ in 0..4 {
		if rules.contains_key(&a) {
			return rules[&a].clone();
		}
		a = rotate(&a);
	}

	println!("ERROR: Not found");
	Vec::new()
}

fn parse_line(s: &str) -> (Vec<Vec<bool>>, Vec<Vec<bool>>) {
	let spl = s.split_whitespace().collect::<Vec<&str>>();
	let mut inp = Vec::new();
	let inp_spl = spl[0].split("/").collect::<Vec<&str>>();
	for s in inp_spl {
		inp.push(s.chars().map(|c| c == '#').collect::<Vec<bool>>());
	}
	let mut outp = Vec::new();
	let outp_spl = spl[2].split("/").collect::<Vec<&str>>();
	for s in outp_spl {
		outp.push(s.chars().map(|c| c == '#').collect::<Vec<bool>>());
	}
	(inp, outp)
}

fn parse_input(path: &str) -> Input {
	let f = File::open(path).unwrap();
	BufReader::new(f).lines().flatten().map(|s| parse_line(&s)).collect()
}

pub fn calc(rules: &Input, iterations: usize) -> usize {
	let mut array = vec!(vec!(false, true, false), vec!(false, false, true), vec!(true, true, true));

	for _ in 0..iterations {
		let multiple = if array.len() % 2 == 0 { 2 } else { 3 };
		let repeats = array.len() / multiple;
		let mut to_combine = Vec::new();
		for i in 0..repeats {
			let mut row = Vec::new();
			for j in 0..repeats {
				let mut sub_arr = Vec::new();
				for y in 0..multiple {
					let mut line = Vec::new();
					for x in 0..multiple {
						line.push(array[i * multiple + y][j * multiple + x]);
					}
					sub_arr.push(line);
				}
				row.push(find_match(&sub_arr, rules));
			}
			to_combine.push(row);
		}

		let mut combined = Vec::new();
		for i in 0..to_combine[0][0].len() * to_combine.len() {
			let mut this_row = Vec::new();
			let major_row = i / to_combine[0][0].len();
			let minor_row = i % to_combine[0][0].len();
			for j in 0..to_combine[major_row].len() {
				this_row.append(&mut to_combine[major_row][j][minor_row].clone());
			}
			combined.push(this_row);
		}
		array = combined;
	}
	
	let mut count = 0;
	for r in array {
		for v in r {
			if v {
				count += 1;
			}
		}
	}
	count
}

fn part1(rules: &Input) -> usize {
	calc(rules, 5)
}

fn part2(rules: &Input) -> usize {
	calc(rules, 18)
}

pub fn main() {
	let rules = parse_input("./input/day21/input.txt");

	let p1_timer = Instant::now();
    let p1_result = part1(&rules);
    let p1_time = p1_timer.elapsed();
	println!("Part 1: {}", p1_result);
	println!("Part 1 Time: {:?}", p1_time);

	let p2_timer = Instant::now();
    let p2_result = part2(&rules);
    let p2_time = p2_timer.elapsed();
	println!("Part 2: {}", p2_result);
	println!("Part 2 Time: {:?}", p2_time);
}