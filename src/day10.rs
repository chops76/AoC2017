use std::io::Read;
use std::fs::File;
use std::time::Instant;
use std::collections::VecDeque;

fn parse_input(path: &str) -> Vec<usize> {
	let mut fstr = String::new();

	File::open(path).unwrap().read_to_string(&mut fstr);
	fstr.split(",").map(|v| v.parse().unwrap()).collect()
}

fn parse_input2(path: &str) -> Vec<usize> {
	let mut fstr = String::new();

	File::open(path).unwrap().read_to_string(&mut fstr);
	let mut ret_vec:Vec<usize> = fstr.as_bytes().iter().map(|v| *v as usize).collect();

	ret_vec.push(17);
	ret_vec.push(31);
	ret_vec.push(73);
	ret_vec.push(47);
	ret_vec.push(23);

	ret_vec
}

fn part1(lengths: &Vec<usize>) -> u32 {
	let mut nums = VecDeque::new();
	for i in 0..256 {
		nums.push_back(i);
	}
	let mut cur_pos = 0;
	let mut skip_size = 0;
	for length in lengths {
		nums.rotate_left(cur_pos);
		nums.make_contiguous()[0..*length].reverse();
		nums.rotate_right(cur_pos);
		cur_pos += length;
		cur_pos += skip_size;
		cur_pos %= nums.len();
		skip_size += 1;
	}
	nums[0] * nums[1]
}

fn part2(lengths: &Vec<usize>) -> String {
	let mut nums:VecDeque<u8> = VecDeque::new();
	for i in 0..=255 {
		nums.push_back(i);
	}
	let mut cur_pos = 0;
	let mut skip_size = 0;

	for _ in 0..64 {
		for length in lengths {
			nums.rotate_left(cur_pos);
			nums.make_contiguous()[0..*length].reverse();
			nums.rotate_right(cur_pos);
			cur_pos += length;
			cur_pos += skip_size;
			cur_pos %= nums.len();
			skip_size += 1;
		}
	}

	let mut ret_str = String::new();

	for set in nums.make_contiguous().chunks(16) {
		let mut xor = set[0];
		for i in 1..set.len() {
			xor ^= set[i];
		}
		ret_str += &format!("{:02x}", xor).to_string();
	}

	ret_str
}

pub fn main() {
	let lengths = parse_input("./input/day10/input.txt");

	let p1_timer = Instant::now();
    let p1_result = part1(&lengths);
    let p1_time = p1_timer.elapsed();
	println!("Part 1: {}", p1_result);
	println!("Part 1 Time: {:?}", p1_time);

	let lengths2 = parse_input2("./input/day10/input.txt");

	let p2_timer = Instant::now();
	let p2_result = part2(&lengths2);
    let p2_time = p2_timer.elapsed();
	println!("Part 2: {}", p2_result);
	println!("Part 2 Time: {:?}", p2_time);
}