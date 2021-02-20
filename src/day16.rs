use std::fs::File;
use std::io::Read;
use std::time::Instant;
use std::collections::HashSet;

type Input = Vec<Command>;

#[derive(Debug)]
enum Command {
	Spin(usize),
	Exchange(usize,usize),
	Partner(char,char)
}

fn parse_line(s: &str) -> Command {
	match s.chars().next().unwrap() {
		's' => { 
			Command::Spin(s[1..].parse().unwrap()) 
		},
		'x' => { 
			let nums = s[1..].split('/').collect::<Vec<&str>>();
			Command::Exchange(nums[0].parse().unwrap(), nums[1].parse().unwrap()) 
		},
		'p' => { 
			let char_vec = s.chars().collect::<Vec<char>>();
			Command::Partner(char_vec[1], char_vec[3]) },
		_ => {
			println!("ILLEGAL COMMAND");
			Command::Spin(0)
		}
	}
}

fn parse_input(path: &str) -> Input {
	let mut fstr = String::new();

	File::open(path).unwrap().read_to_string(&mut fstr);
	fstr.split(",").map(|s| parse_line(s)).collect()
}

fn part1(commands: &Input) -> String {
	let mut prog_order = Vec::new();
	let mut head = 0;
	for c in 'a'..='p' {
		prog_order.push(c);
	}
	let progs = prog_order.len();

	for cmd in commands {
		match cmd {
			Command::Spin(val) => {
				head = (head + progs - val) % progs;
			},
			Command::Exchange(pos1, pos2) => {
				let tmp = prog_order[(head + pos1) % progs];
				prog_order[(head + pos1) % progs] = prog_order[(head + pos2) % progs];
				prog_order[(head + pos2) % progs] = tmp;
			},
			Command::Partner(c1, c2) => {
				let pos1 = prog_order.iter().position(|&c| c == *c1).unwrap();
				let pos2 = prog_order.iter().position(|&c| c == *c2).unwrap();
				let tmp = prog_order[pos1];
				prog_order[pos1] = prog_order[pos2];
				prog_order[pos2] = tmp;
			}
		}
	}
	prog_order.iter().cycle().skip(head).take(progs).collect::<String>()
}

fn part2(commands: &Input) -> String {
	let mut prog_order = Vec::new();
	let mut count = 0;
	let mut head = 0;
	let mut cache = HashSet::new();
	let mut seen = Vec::new();
	for c in 'a'..='p' {
		prog_order.push(c);
	}
	let progs = prog_order.len();

	loop {
		let copy = prog_order.clone();
		seen.push(prog_order.clone());
		if cache.contains(&(copy, head)) {
			break;
		}
		cache.insert((prog_order.clone(), head));
		for cmd in commands {
			match cmd {
				Command::Spin(val) => {
					head = (head + progs - val) % progs;
				},
				Command::Exchange(pos1, pos2) => {
					let tmp = prog_order[(head + pos1) % progs];
					prog_order[(head + pos1) % progs] = prog_order[(head + pos2) % progs];
					prog_order[(head + pos2) % progs] = tmp;
				},
				Command::Partner(c1, c2) => {
					let pos1 = prog_order.iter().position(|&c| c == *c1).unwrap();
					let pos2 = prog_order.iter().position(|&c| c == *c2).unwrap();
					let tmp = prog_order[pos1];
					prog_order[pos1] = prog_order[pos2];
					prog_order[pos2] = tmp;
				}
			}
		}
		count += 1;
	}
	seen[1000000000 % count].iter().cycle().skip(head).take(progs).collect::<String>()
}

pub fn main() {
	let commands = parse_input("./input/day16/input.txt");

	let p1_timer = Instant::now();
    let p1_result = part1(&commands);
    let p1_time = p1_timer.elapsed();
	println!("Part 1: {}", p1_result);
	println!("Part 1 Time: {:?}", p1_time);

	let p2_timer = Instant::now();
    let p2_result = part2(&commands);
    let p2_time = p2_timer.elapsed();
	println!("Part 2: {}", p2_result);
	println!("Part 2 Time: {:?}", p2_time);
}