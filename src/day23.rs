use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;
use std::collections::HashMap;
use primes::is_prime;

type Input = Vec<Command>;

#[derive(Debug)]
#[derive(Clone)]
pub enum Command {
	Set(String, String),
	Sub(String, String),
	Mul(String, String),
	Jnz(String, String)
}

fn parse_line(s: &str) -> Command {
	let spl = s.split_whitespace().collect::<Vec<&str>>();
	match spl[0] {
		"set" => { 
			Command::Set(spl[1].to_string(), spl[2].to_string())
		},
		"sub" => { 
			Command::Sub(spl[1].to_string(), spl[2].to_string()) 
		},
		"mul" => {
			Command::Mul(spl[1].to_string(), spl[2].to_string())
		},
		"jnz" => {
			Command::Jnz(spl[1].to_string(), spl[2].to_string())
		},
		_ => {
			println!("ILLEGAL COMMAND {}", spl[0]);
			Command::Set("0".to_string(), "0".to_string())
		}
	}
}

fn parse_input(path: &str) -> Input {
	let f = File::open(path).unwrap();
	BufReader::new(f).lines().flatten().map(|s| parse_line(&s)).collect()
}

fn part1(commands: &Input) -> i64 {
	let mut regs:HashMap<String, i64> = HashMap::new();
	for r in 'a'..='h' {
		regs.insert(r.to_string(), 0);
	}
	let mut muls = 0;
	let mut ip: i64 = 0;

	while ip >= 0 && ip < commands.len() as i64 {
		match &commands[ip as usize] {
			Command::Set(x, y) => {
				let val = match y.parse() { Ok(v) => v, Err(_) => regs[y] };
				regs.insert(x.to_string(), val);
				ip += 1;
			},
			Command::Mul(x, y) => {
				muls += 1;
				let val1 = regs[x];
				let val2 = match y.parse() { Ok(v) => v, Err(_) => regs[y] };
				regs.insert(x.to_string(), val1 * val2);
				ip += 1;
			},
			Command::Jnz(x, y) => {
				let val = match x.parse() { Ok(v) => v, Err(_) => regs[x] };
				if val != 0 {
					let offset =  match y.parse() { Ok(v) => v, Err(_) => regs[y] };
					ip += offset;
				} else {
					ip += 1;
				}
			}
			Command::Sub(x, y) => {
				let val1 = regs[x];
				let val2 = match y.parse() { Ok(v) => v, Err(_) => regs[y] };
				regs.insert(x.to_string(), val1 - val2);
				ip += 1;
			},
			_ => {
				println!("Invalid Command: {:?}", commands[ip as usize]);
				break;
			}
		}

	}

	muls
}

fn part2() -> usize {
	let mut count = 0;
	for a in (109300..=126300).step_by(17) {
		if !is_prime(a) {
			count += 1;
		}
	}

	count
}

pub fn main() {
	let commands = parse_input("./input/day23/input.txt");

	let p1_timer = Instant::now();
    let p1_result = part1(&commands);
    let p1_time = p1_timer.elapsed();
	println!("Part 1: {}", p1_result);
	println!("Part 1 Time: {:?}", p1_time);

	let p2_timer = Instant::now();
    let p2_result = part2();
    let p2_time = p2_timer.elapsed();
	println!("Part 2: {}", p2_result);
	println!("Part 2 Time: {:?}", p2_time);
}