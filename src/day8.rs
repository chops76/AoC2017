use std::fs::File;
use std::time::Instant;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;
use std::cmp;

#[derive(Debug)]
struct Instruction {
	mod_name: String,
	mod_amount: i64,
	cmp_name: String,
	cmp_amount: i64,
	cmp_type: String
}

type Input = Vec<Instruction>;

fn parse_line(s: &str) -> Instruction {
	let spl = s.split_whitespace().collect::<Vec<&str>>();
	Instruction {
		mod_name: spl[0].to_string(),
		mod_amount: spl[2].parse::<i64>().unwrap() * if spl[1] == "dec" { -1 } else { 1 },
		cmp_name: spl[4].to_string(),
		cmp_amount: spl[6].parse().unwrap(),
		cmp_type: spl[5].to_string()
	}
}

fn parse_input(path: &str) -> Input {
	let f = File::open(path).unwrap();
	BufReader::new(f).lines().flatten().map(|s| parse_line(&s)).collect()
}

fn do_compare(a: i64, b: i64, c: &str) -> bool {
	match c {
		"<" => { a < b },
		">" => { a > b },
		"<=" => { a <= b },
		">=" => { a >= b },
		"==" => { a == b },
		"!=" => { a != b },
		_ => { println!("BAD OPERATOR"); false }
	}
}

fn part1(instr: &Input) -> i64 {
	let mut regs:HashMap<String, i64> = HashMap::new();
	for inst in instr {
		if !regs.contains_key(&inst.mod_name) {
			regs.insert(inst.mod_name.clone(), 0);
		}
		if !regs.contains_key(&inst.cmp_name) {
			regs.insert(inst.cmp_name.clone(), 0);
		}
		if do_compare(regs[&inst.cmp_name], inst.cmp_amount, &inst.cmp_type) {
			let cur_val = regs[&inst.mod_name];
			regs.insert(inst.mod_name.clone(), cur_val + inst.mod_amount);
		}
	}
	*regs.iter().max_by_key(|(_,&v)| v).unwrap().1
}

fn part2(instr: &Input) -> i64 {
	let mut largest = 0;
	let mut regs:HashMap<String, i64> = HashMap::new();
	for inst in instr {
		if !regs.contains_key(&inst.mod_name) {
			regs.insert(inst.mod_name.clone(), 0);
		}
		if !regs.contains_key(&inst.cmp_name) {
			regs.insert(inst.cmp_name.clone(), 0);
		}
		if do_compare(regs[&inst.cmp_name], inst.cmp_amount, &inst.cmp_type) {
			let cur_val = regs[&inst.mod_name];
			regs.insert(inst.mod_name.clone(), cur_val + inst.mod_amount);
		}
		largest = cmp::max(largest, *regs.iter().max_by_key(|(_,&v)| v).unwrap().1);
	}
	largest
}

pub fn main() {
	let instr = parse_input("./input/day8/input.txt");

	let p1_timer = Instant::now();
    let p1_result = part1(&instr);
    let p1_time = p1_timer.elapsed();
	println!("Part 1: {}", p1_result);
	println!("Part 1 Time: {:?}", p1_time);

	let p2_timer = Instant::now();
    let p2_result = part2(&instr);
    let p2_time = p2_timer.elapsed();
	println!("Part 2: {}", p2_result);
	println!("Part 2 Time: {:?}", p2_time);
}
