use std::io::Read;
use std::fs::File;
use std::time::Instant;

fn parse_input(path: &str) -> String {
	let mut fstr = String::new();

	File::open(path).unwrap().read_to_string(&mut fstr);
	fstr
}

fn part1(string: &str) -> u32 {
	let mut total = 0;
	let mut depth = 0;
	let mut pos = 0;
	let mut garbage = false;
	let chrs = string.chars().collect::<Vec<char>>();
	while pos < chrs.len() {
		if garbage {
			if chrs[pos] == '!' {
				pos += 2;
			} else {
				if chrs[pos] == '>' {
					garbage = false;
				}
				pos += 1;
			}
		} else {
			if chrs[pos] == '<' {
				garbage = true;
			} else if chrs[pos] == '{' {
				depth += 1;
			} else if chrs[pos] == '}' {
				total += depth;
				depth -= 1;
			}
			pos += 1;
		}
	}

	total
}

fn part2(string: &str) -> u32 {
	let mut total = 0;
	let mut pos = 0;
	let mut garbage = false;
	let chrs = string.chars().collect::<Vec<char>>();
	while pos < chrs.len() {
		if garbage {
			if chrs[pos] == '!' {
				pos += 2;
			} else if chrs[pos] == '>' {
				garbage = false;
				pos += 1;
			} else {
				total += 1;
				pos += 1;
			}
		} else {
			if chrs[pos] == '<' {
				garbage = true;
			} 
			pos += 1;
		}
	}

	total
}

pub fn main() {
	let string = parse_input("./input/day9/input.txt");

	let p1_timer = Instant::now();
    let p1_result = part1(&string);
    let p1_time = p1_timer.elapsed();
	println!("Part 1: {}", p1_result);
	println!("Part 1 Time: {:?}", p1_time);

	let p2_timer = Instant::now();
	let p2_result = part2(&string);
    let p2_time = p2_timer.elapsed();
	println!("Part 2: {}", p2_result);
	println!("Part 2 Time: {:?}", p2_time);
}