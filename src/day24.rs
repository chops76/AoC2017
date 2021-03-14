use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;
use std::collections::VecDeque;

type Input = VecDeque<(u32, u32)>;

fn parse_line(s: &str) -> (u32, u32) {
	let spl = s.split('/').collect::<Vec<&str>>();
	(spl[0].parse().unwrap(), spl[1].parse().unwrap())
}

fn parse_input(path: &str) -> Input {
	let f = File::open(path).unwrap();
	BufReader::new(f).lines().flatten().map(|s| parse_line(&s)).collect()
}

fn recur(end_val: u32, cur_list: &mut VecDeque<(u32, u32)>, parts: &Input ) -> u32 {
	let options = parts.iter().enumerate().filter(|(_, v)| v.0 == end_val || v.1 == end_val)
	                   .collect::<Vec<(usize, &(u32, u32))>>();
	if options.len() == 0 {
		return cur_list.iter().map(|v| v.0 + v.1).sum();
	}
	let mut max_val = 0;
	for o in options {
		let end_pt = if o.1.0 == end_val {o.1.1} else {o.1.0};
		cur_list.push_back(*o.1);
		let mut p_temp = parts.clone();
		p_temp.remove(o.0);
		let val = recur(end_pt, cur_list, &p_temp);
		if val > max_val {
			max_val = val;
		}
		cur_list.pop_back();
	}
	max_val
}

fn part1(parts: &Input) -> u32 {
	let mut cur_list = VecDeque::new();
	recur(0, &mut cur_list, &parts)
}

fn recur2(end_val: u32, cur_list: &mut VecDeque<(u32, u32)>, parts: &Input ) -> (usize, u32) {
	let options = parts.iter().enumerate().filter(|(_, v)| v.0 == end_val || v.1 == end_val)
	                   .collect::<Vec<(usize, &(u32, u32))>>();
	if options.len() == 0 {
		return (cur_list.len(), cur_list.iter().map(|v| v.0 + v.1).sum());
	}
	let mut max_val = (0, 0);
	for o in options {
		let end_pt = if o.1.0 == end_val {o.1.1} else {o.1.0};
		cur_list.push_back(*o.1);
		let mut p_temp = parts.clone();
		p_temp.remove(o.0);
		let val = recur2(end_pt, cur_list, &p_temp);
		if val.0 > max_val.0 || (val.0 == max_val.0 && val.1 > max_val.1) {
			max_val = val;
		}
		cur_list.pop_back();
	}
	max_val
}

fn part2(parts: &Input) -> u32 {
	let mut cur_list = VecDeque::new();
	recur2(0, &mut cur_list, &parts).1
}

pub fn main() {
	let parts = parse_input("./input/day24/input.txt");

	let p1_timer = Instant::now();
    let p1_result = part1(&parts);
    let p1_time = p1_timer.elapsed();
	println!("Part 1: {}", p1_result);
	println!("Part 1 Time: {:?}", p1_time);

	let p2_timer = Instant::now();
    let p2_result = part2(&parts);
    let p2_time = p2_timer.elapsed();
	println!("Part 2: {}", p2_result);
	println!("Part 2 Time: {:?}", p2_time);
}