use std::time::Instant;

fn part1(num: i32) -> i32 {
	let mut dir = 0;
	let mut max_x = 0;
	let mut max_y = 0;
	let mut min_x = 0;
	let mut min_y = 0;
	let mut cur_x:i32 = 0;
	let mut cur_y:i32 = 0;
	for _ in 1..num {
		let mut new_dir = dir;
		match dir {
			0 => {
				cur_x += 1;
				if cur_x > max_x {
					max_x = cur_x;
					new_dir += 1;
					new_dir %= 4;
				}
			},
			1 => {
				cur_y += 1;
				if cur_y > max_y {
					max_y = cur_y;
					new_dir += 1;
					new_dir %= 4;
				}
			},
			2 => {
				cur_x -= 1;
				if cur_x < min_x {
					min_x = cur_x;
					new_dir += 1;
					new_dir %= 4;
				}
			},
			3 => {
				cur_y -= 1;
				if cur_y < min_y {
					min_y = cur_y;
					new_dir += 1;
					new_dir %= 4;
				}
			},
			_ => {
				println!("ERROR");
			}
		}
		dir = new_dir;
	}
	cur_x.abs() + cur_y.abs()
}

fn sum(x: i32, y: i32, nums: &Vec<Vec<u32>>) -> u32 {
	let mut total = 0;
	for i in -1..=1 {
		for j in -1..=1 {
			total += nums[(x + 100 + i) as usize][(y + 100 + j) as usize];
		}
	}
	total
}

fn part2(num: i32) -> i32 {
	let mut dir = 0;
	let mut max_x = 0;
	let mut max_y = 0;
	let mut min_x = 0;
	let mut min_y = 0;
	let mut cur_x:i32 = 0;
	let mut cur_y:i32 = 0;
	let mut nums:Vec<Vec<u32>> = vec![vec![0;200];200];
	nums[100][100] = 1;
	loop {
		let mut new_dir = dir;
		match dir {
			0 => {
				cur_x += 1;
				if cur_x > max_x {
					max_x = cur_x;
					new_dir += 1;
					new_dir %= 4;
				}
			},
			1 => {
				cur_y += 1;
				if cur_y > max_y {
					max_y = cur_y;
					new_dir += 1;
					new_dir %= 4;
				}
			},
			2 => {
				cur_x -= 1;
				if cur_x < min_x {
					min_x = cur_x;
					new_dir += 1;
					new_dir %= 4;
				}
			},
			3 => {
				cur_y -= 1;
				if cur_y < min_y {
					min_y = cur_y;
					new_dir += 1;
					new_dir %= 4;
				}
			},
			_ => {
				println!("ERROR");
			}
		}
		let new_sum = sum(cur_x, cur_y, &nums);
		if new_sum > num as u32 {
			return new_sum as i32;
		}
		nums[(cur_x + 100) as usize][(cur_y + 100) as usize] = new_sum;
		dir = new_dir;
	}
}


pub fn main() {
	let p1_timer = Instant::now();
	let p1_result = part1(277678);
    let p1_time = p1_timer.elapsed();
	println!("Part 1: {}", p1_result);
	println!("Part 1 Time: {:?}", p1_time);

	let p2_timer = Instant::now();
    let p2_result = part2(277678);
    let p2_time = p2_timer.elapsed();
	println!("Part 2: {}", p2_result);
	println!("Part 2 Time: {:?}", p2_time);
}
