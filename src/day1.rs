use std::io::Read;
use std::fs::File;
use std::time::Instant;

fn parse_input(path: &str) -> Vec<u32> {
	let mut fstr = String::new();

	File::open(path).unwrap().read_to_string(&mut fstr);
	fstr.chars().map(|c| c.to_digit(10).unwrap()).collect()
}

fn part1(vals: &Vec<u32>) -> u32 {
    vals.iter().zip(vals.iter().cycle().skip(1)).filter(|(a,b)|a==b).map(|(a,_)| a).sum()
}

fn part2(vals: &Vec<u32>) -> u32 {
    vals.iter().zip(vals.iter().cycle().skip(vals.len()/2)).filter(|(a,b)|a==b).map(|(a,_)| a).sum()
}

pub fn main() {
    let vals = parse_input("./input/day1/input.txt");

	let p1_timer = Instant::now();
    let p1_result = part1(&vals);
    let p1_time = p1_timer.elapsed();
	println!("Part 1: {}", p1_result);
	println!("Part 1 Time: {:?}", p1_time);

	let p2_timer = Instant::now();
    let p2_result = part2(&vals);
    let p2_time = p2_timer.elapsed();
	println!("Part 2: {}", p2_result);
	println!("Part 2 Time: {:?}", p2_time);
}
