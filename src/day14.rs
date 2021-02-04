use std::time::Instant;
use std::collections::VecDeque;
use std::collections::HashSet;

fn get_lengths(key: &str) -> Vec<usize> {
	let mut ret_vec:Vec<usize> = key.as_bytes().iter().map(|v| *v as usize).collect();

	ret_vec.push(17);
	ret_vec.push(31);
	ret_vec.push(73);
	ret_vec.push(47);
	ret_vec.push(23);

	ret_vec
}

fn run_hash(key: &str) -> String {
	let lengths = get_lengths(key);
	let mut nums:VecDeque<u8> = VecDeque::new();
	for i in 0..=255 {
		nums.push_back(i);
	}
	let mut cur_pos = 0;
	let mut skip_size = 0;

	for _ in 0..64 {
		for length in &lengths {
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
		ret_str += &format!("{:08b}", xor).to_string();
	}

	ret_str
}

fn part1(key: &str) -> usize {
	let mut count = 0;
	for i in 0..128 {
		let new_key = format!("{}-{}", key, i);
		let hash = run_hash(&new_key);
		count += hash.chars().filter(|c| *c == '1').count();
	}
	count
}

fn set_with_node(nodes: &HashSet<(usize, usize)>, node: (usize, usize)) -> HashSet<(usize, usize)> {
	let mut queue = VecDeque::new();
	let mut visited = HashSet::new();
	queue.push_back(node);
	visited.insert(node);
	while queue.len() != 0 {
		let (x, y) = queue.pop_front().unwrap();

		if x > 0 && nodes.contains(&(x-1,y)) {
			if !visited.contains(&(x-1,y)) {
				queue.push_back((x-1,y));
				visited.insert((x-1,y));
			}
		}
		if y > 0 && nodes.contains(&(x,y-1)) {
			if !visited.contains(&(x,y-1)) {
				queue.push_back((x,y-1));
				visited.insert((x,y-1));
			}
		}
		if nodes.contains(&(x+1,y)) {
			if !visited.contains(&(x+1,y)) {
				queue.push_back((x+1,y));
				visited.insert((x+1,y));
			}
		}
		if nodes.contains(&(x,y+1)) {
			if !visited.contains(&(x,y+1)) {
				queue.push_back((x,y+1));
				visited.insert((x,y+1));
			}
		}
	}
	visited
}

fn part2(key: &str) -> usize {
	let mut nodes = HashSet::new();
	for i in 0..128 {
		let new_key = format!("{}-{}", key, i);
		let hash = run_hash(&new_key);
		let chrs = hash.chars().collect::<Vec<char>>();
		for j in 0..chrs.len() {
			if chrs[j] == '1' {
				nodes.insert((j, i));
			}
		}
	}
	let mut count = 0;

	while nodes.len() != 0 {
		count += 1;
		let group = set_with_node(&nodes, *nodes.iter().next().unwrap());
		for i in group {
			nodes.remove(&i);
		}
	}
	count
}

pub fn main() {

	let p1_timer = Instant::now();
	let p1_result = part1("flqrgnkx");
    let p1_time = p1_timer.elapsed();
	println!("Part 1: {}", p1_result);
	println!("Part 1 Time: {:?}", p1_time);

	let p2_timer = Instant::now();
	let p2_result = part2("hxtvlmkl");
	let p2_time = p2_timer.elapsed();
	println!("Part 2: {}", p2_result);
	println!("Part 2 Time: {:?}", p2_time);
}