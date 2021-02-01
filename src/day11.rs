use std::io::Read;
use std::fs::File;
use std::time::Instant;
use std::cmp;

fn parse_input(path: &str) -> Vec<String> {
	let mut fstr = String::new();

	File::open(path).unwrap().read_to_string(&mut fstr);
	fstr.split(",").map(|s| s.to_string()).collect()
}

fn part1(directions: &Vec<String>) -> usize
{
	let mut x: i32 = 0;
	let mut y: i32 = 0;
	let mut z: i32 = 0;

	for d in directions {
		match d.as_str() {
			"n" => { y += 1; z -= 1 },
			"ne" => { x += 1; z -= 1 },
			"se" => { x += 1; y -= 1 },
			"s" => { y -= 1; z += 1 },
			"sw" => { x -= 1; z += 1 },
			"nw" => { x -= 1; y += 1 },
			_ => { println!("BAD DIR") }
		}
	}
	(x.abs() + y.abs() + z.abs()) as usize / 2
}

fn part2(directions: &Vec<String>) -> usize
{
	let mut x: i32 = 0;
	let mut y: i32 = 0;
	let mut z: i32 = 0;
	let mut furthest = 0;

	for d in directions {
		match d.as_str() {
			"n" => { y += 1; z -= 1 },
			"ne" => { x += 1; z -= 1 },
			"se" => { x += 1; y -= 1 },
			"s" => { y -= 1; z += 1 },
			"sw" => { x -= 1; z += 1 },
			"nw" => { x -= 1; y += 1 },
			_ => { println!("BAD DIR") }
		}
		furthest = cmp::max(furthest, (x.abs() + y.abs() + z.abs()) as usize / 2);
	}
	furthest
}

pub fn main() {
	let directions = parse_input("./input/day11/input.txt");

	let p1_timer = Instant::now();
    let p1_result = part1(&directions);
    let p1_time = p1_timer.elapsed();
	println!("Part 1: {}", p1_result);
	println!("Part 1 Time: {:?}", p1_time);

	let p2_timer = Instant::now();
	let p2_result = part2(&directions);
    let p2_time = p2_timer.elapsed();
	println!("Part 2: {}", p2_result);
	println!("Part 2 Time: {:?}", p2_time);
}