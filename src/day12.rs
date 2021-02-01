use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;
use std::collections::VecDeque;
use std::collections::HashSet;

fn parse_line(s: &str) -> Vec<usize> {
	let spl = s.split(" <-> ").collect::<Vec<&str>>();
	spl[1].split(", ").map(|v| v.parse().unwrap()).collect()
}

fn parse_input(path: &str) -> Vec<Vec<usize>> {
	let f = File::open(path).unwrap();
	
	BufReader::new(f).lines().flatten().map(|s| parse_line(&s)).collect()
}

fn set_with_node(nodes: &Vec<Vec<usize>>, node_num: usize) -> HashSet<usize> {
	let mut queue = VecDeque::new();
	let mut visited = HashSet::new();
	queue.push_back(node_num);
	visited.insert(node_num);
	while queue.len() != 0 {
		let node = queue.pop_front().unwrap();
		
		for neighbor in &nodes[node] {
			if !visited.contains(neighbor) {
				queue.push_back(*neighbor);
				visited.insert(*neighbor);
			}
		}
	}
	visited
}

fn part1(nodes: &Vec<Vec<usize>>) -> usize {
	let group = set_with_node(nodes, 0);
	group.len()
}

fn part2(nodes: &Vec<Vec<usize>>) -> usize {
	let mut count = 0;
	let mut all_nodes = HashSet::new();
	for i in 0..nodes.len() {
		all_nodes.insert(i);
	}
	while all_nodes.len() != 0 {
		count += 1;
		let group = set_with_node(nodes, *all_nodes.iter().next().unwrap());
		for i in group {
			all_nodes.remove(&i);
		}
	}
	count
}

pub fn main() {
	let nodes = parse_input("./input/day12/input.txt");

	let p1_timer = Instant::now();
    let p1_result = part1(&nodes);
    let p1_time = p1_timer.elapsed();
	println!("Part 1: {}", p1_result);
	println!("Part 1 Time: {:?}", p1_time);

	let p2_timer = Instant::now();
    let p2_result = part2(&nodes);
    let p2_time = p2_timer.elapsed();
	println!("Part 2: {}", p2_result);
	println!("Part 2 Time: {:?}", p2_time);
}