use std::time::Instant;
use std::collections::VecDeque;

fn part1(skip: usize) -> u32 {
	let mut buf = VecDeque::new();
	buf.push_back(0);
	let mut cur_pos = 0;
	for i in 1..=2017 {
		cur_pos = (cur_pos + skip) % buf.len();
		buf.insert(cur_pos + 1, i);
		cur_pos += 1;
	}
	buf[(cur_pos + 1) % buf.len()]
}

fn part2(skip: usize) -> u32 {
	let mut size = 1;
	let mut cur_pos = 0;
	let mut val = 0;
	for i in 1..=50000000 {
		cur_pos = (cur_pos + skip) % size;
		if cur_pos == 0 {
			val = i;
		}
		cur_pos += 1;
		size += 1;
	}
	val
}

pub fn main() {
	let p1_timer = Instant::now();
    let p1_result = part1(377);
    let p1_time = p1_timer.elapsed();
	println!("Part 1: {}", p1_result);
	println!("Part 1 Time: {:?}", p1_time);

	let p2_timer = Instant::now();
    let p2_result = part2(377);
    let p2_time = p2_timer.elapsed();
	println!("Part 2: {}", p2_result);
	println!("Part 2 Time: {:?}", p2_time);
}