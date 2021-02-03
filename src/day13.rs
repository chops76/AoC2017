use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;
use std::collections::HashMap;

fn parse_line(s: &str) -> (usize, usize) {
	let spl = s.split(": ").collect::<Vec<&str>>();
	(spl[0].parse().unwrap(), spl[1].parse().unwrap())
}

fn parse_input(path: &str) -> HashMap<usize, usize> {
	let f = File::open(path).unwrap();
	let mut hm = HashMap::new();
	
	let vals:Vec<(usize, usize)> = BufReader::new(f).lines().flatten().map(|s| parse_line(&s)).collect();
	for (key, val) in vals {
		hm.insert(key, val);
	}

	hm
}

fn calc_score(scanners: &HashMap<usize, usize>) -> usize {
	let mut scan_pos = HashMap::new();
	let mut going_down = HashMap::new();
	let mut score = 0;

	for (key, _) in scanners {
		scan_pos.insert(key, 0);
		going_down.insert(key, false);
	}

	let end_pos = scan_pos.iter().map(|(k, _)| **k).max().unwrap();
	for i in 0..=end_pos {
		if scan_pos.contains_key(&i) && scan_pos[&i] == 0 {
			score += i * scanners[&i];
		}

		for (scanner, scan_size) in scanners {
			let cur_pos = scan_pos[scanner];
			if cur_pos == 0 || cur_pos == scan_size - 1 {
				let down = going_down[scanner];
				going_down.insert(scanner, !down);
			}
			if going_down[scanner] {
				scan_pos.insert(scanner, cur_pos + 1);
			} else {
				scan_pos.insert(scanner, cur_pos - 1);
			}
		}
	}

	score
}


fn part1(scanners: &HashMap<usize, usize>) -> usize {
	calc_score(scanners)
}

fn part2(scanners: &HashMap<usize, usize>) -> usize {
	let mut valid = vec![true; 10000000];
	for (range, size) in scanners {
		let period = (size-1)*2;
		let mut first:i32 = period as i32 - (*range as i32);
		if *range == 0 {
			first = 0;
		}
		while first < 0 {
			first += period as i32;
		}
		let mut idx = first as usize;
		while idx < 10000000 {
			valid[idx] = false;
			idx += period;
		}
	}
	for i in 0..valid.len() {
		if valid[i] == true {
			return i;
		}
	}
	println!("ERROR: Not found");
	0
}

pub fn main() {
	let scanners = parse_input("./input/day13/input.txt");

	let p1_timer = Instant::now();
    let p1_result = part1(&scanners);
    let p1_time = p1_timer.elapsed();
	println!("Part 1: {}", p1_result);
	println!("Part 1 Time: {:?}", p1_time);

	let p2_timer = Instant::now();
    let p2_result = part2(&scanners);
    let p2_time = p2_timer.elapsed();
	println!("Part 2: {}", p2_result);
	println!("Part 2 Time: {:?}", p2_time);
}