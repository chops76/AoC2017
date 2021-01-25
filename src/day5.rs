use std::io::Read;
use std::fs::File;
use std::time::Instant;

fn parse_input(path: &str) -> Vec<i32> {
	let mut fstr = String::new();

	File::open(path).unwrap().read_to_string(&mut fstr);
	fstr.split_whitespace().map(|s| s.parse().unwrap()).collect()
}

fn part1(jumps_ref: &Vec<i32>) -> usize {
	let mut jumps = jumps_ref.clone();
	let mut count = 0;
	let mut ip: i32 = 0;
	while ip >= 0 && ip < jumps.len() as i32 {
		let new_ip = ip + jumps[ip as usize];
		jumps[ip as usize] += 1;
		ip = new_ip;
		count += 1;
	}
	count
}

fn part2(jumps_ref: &Vec<i32>) -> usize {
	let mut jumps = jumps_ref.clone();
	let mut count = 0;
	let mut ip: i32 = 0;
	while ip >= 0 && ip < jumps.len() as i32 {
		let new_ip = ip + jumps[ip as usize];
		jumps[ip as usize] += if jumps[ip as usize] >= 3 { -1 } else { 1 };
		ip = new_ip;
		count += 1;
	}
	count
}

pub fn main() {
	let jumps = parse_input("./input/day5/input.txt");

	let p1_timer = Instant::now();
    let p1_result = part1(&jumps);
    let p1_time = p1_timer.elapsed();
	println!("Part 1: {}", p1_result);
	println!("Part 1 Time: {:?}", p1_time);

	let p2_timer = Instant::now();
    let p2_result = part2(&jumps);
    let p2_time = p2_timer.elapsed();
	println!("Part 2: {}", p2_result);
	println!("Part 2 Time: {:?}", p2_time);
}
