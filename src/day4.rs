use std::fs::File;
use std::time::Instant;
use std::io::{BufRead, BufReader};
use std::collections::HashSet;
use std::cmp;

type Input = Vec<Vec<String>>;

fn parse_line(s: &str) -> Vec<String>{
	s.split_whitespace().map(|s| s.to_string()).collect()
}

fn parse_input(path: &str) -> Input {
	let f = File::open(path).unwrap();
	BufReader::new(f).lines().flatten().map(|s| parse_line(&s)).collect()
}

fn part1(word_lists: &Input) -> usize {
	word_lists.iter()
			  .filter(|l| l.len() == l.iter().collect::<HashSet<&String>>().len())
			  .count()
}

fn part2(word_lists: &Input) -> usize {
	word_lists.iter()
			  .filter(|l| l.len() == l.iter().map(|w| { 
				  let mut tmp = w.chars().collect::<Vec<char>>(); 
				  tmp.sort(); 
				  tmp.iter().collect::<String>()})
			  .collect::<HashSet<String>>().len())
			  .count()
}

pub fn main() {
	let word_lists = parse_input("./input/day4/input.txt");

	let p1_timer = Instant::now();
    let p1_result = part1(&word_lists);
    let p1_time = p1_timer.elapsed();
	println!("Part 1: {}", p1_result);
	println!("Part 1 Time: {:?}", p1_time);

	let p2_timer = Instant::now();
    let p2_result = part2(&word_lists);
    let p2_time = p2_timer.elapsed();
	println!("Part 2: {}", p2_result);
	println!("Part 2 Time: {:?}", p2_time);
}
