use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;
use std::collections::HashMap;
use std::collections::HashSet;

fn parse_line(s: &str, hm: &mut HashMap<String, u32>, children: &mut HashMap<String, Vec<String>>) {
	let spl = s.split(" -> ").collect::<Vec<&str>>();
	let left = spl[0].split(" (").collect::<Vec<&str>>();
	hm.insert(left[0].to_string(), left[1][..left[1].len()-1].parse().unwrap());
	if spl.len() > 1 {
		let right = spl[1].split(", ").map(|s| s.to_string()).collect::<Vec<String>>();
		children.insert(left[0].to_string(), right);
	}
}

fn parse_input(path: &str) -> (HashMap<String, u32>, HashMap<String, Vec<String>>) {
	let f = File::open(path).unwrap();
	let mut hm = HashMap::new();
	let mut children = HashMap::new();
	for line in BufReader::new(f).lines().flatten() {
		parse_line(&line, &mut hm, &mut children);
	}
	(hm, children)
}

fn find_head(children: &HashMap<String, Vec<String>>) -> String {
	let mut hs = HashSet::new();
	for (_, clist) in children {
		for c in clist {
			hs.insert(c.clone());
		}
	}
	for (parent, _) in children {
		if !hs.contains(parent) {
			return parent.clone();
		}
	}

	"Error".to_string()
}

fn part1(children: &HashMap<String, Vec<String>>) -> String {
	find_head(children)
}

fn part2(children: &HashMap<String, Vec<String>>, weight: &HashMap<String, u32>) -> u32 {
	//let head = find_head(children);
	let head = "dqwocyn".to_string();
	println!("{} internal weight is {}", head, weight[&head]);
	for c in &children[&head] {
		println!("Weight of {} is {}", c, calc_weight(c, children, weight));
	}
	0
}

fn calc_weight(name: &str, children: &HashMap<String, Vec<String>>, weight: &HashMap<String, u32>) -> u32 {
	let mut ret_val = weight[name];
	if children.contains_key(name) {
		for c in &children[name] {
			ret_val += calc_weight(c, children, weight);
		}
	}
	ret_val
}

pub fn main() {
	let (weights, children) = parse_input("./input/day7/input.txt");

	let p1_timer = Instant::now();
    let p1_result = part1(&children);
    let p1_time = p1_timer.elapsed();
	println!("Part 1: {}", p1_result);
	println!("Part 1 Time: {:?}", p1_time);

	let p2_timer = Instant::now();
    let p2_result = part2(&children, &weights);
    let p2_time = p2_timer.elapsed();
	println!("Part 2: {}", p2_result);
	println!("Part 2 Time: {:?}", p2_time);
}