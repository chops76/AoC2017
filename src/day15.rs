use std::time::Instant;

fn gen_a(num: u64) -> u64 {
	num * 16807 % 2147483647
}

fn gen_b(num: u64) -> u64 {
	num * 48271 % 2147483647
}

fn gen_a_mult(num: u64) -> u64 {
	let mut new_num = num * 16807 % 2147483647;
	while new_num % 4 != 0 {
		new_num = new_num * 16807 % 2147483647;
	}
	new_num
}

fn gen_b_mult(num: u64) -> u64 {
	let mut new_num = num * 48271 % 2147483647;
	while new_num % 8 != 0 {
		new_num = new_num * 48271 % 2147483647;
	}
	new_num
}

fn part1(val1: u64, val2: u64) -> usize {
	let mut count = 0;
	let mut cur_val_a = val1;
	let mut cur_val_b = val2;
	for _ in 0..40000000 {
		cur_val_a = gen_a(cur_val_a);
		cur_val_b = gen_b(cur_val_b);
		if cur_val_a & 0xffff == cur_val_b & 0xffff {
			count += 1;
		}
	}
	count
}

fn part2(val1: u64, val2: u64) -> usize {
	let mut count = 0;
	let mut cur_val_a = val1;
	let mut cur_val_b = val2;
	for _ in 0..5000000 {
		cur_val_a = gen_a_mult(cur_val_a);
		cur_val_b = gen_b_mult(cur_val_b);
		if cur_val_a & 0xffff == cur_val_b & 0xffff {
			count += 1;
		}
	}
	count
}

pub fn main() {

	let p1_timer = Instant::now();
	let p1_result = part1(883, 879);
    let p1_time = p1_timer.elapsed();
	println!("Part 1: {}", p1_result);
	println!("Part 1 Time: {:?}", p1_time);

	let p2_timer = Instant::now();
	let p2_result = part2(883, 879);
	let p2_time = p2_timer.elapsed();
	println!("Part 2: {}", p2_result);
	println!("Part 2 Time: {:?}", p2_time);
}