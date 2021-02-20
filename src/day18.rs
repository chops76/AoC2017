use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;
use std::collections::HashMap;
use std::collections::VecDeque;

type Input = Vec<Command>;

#[derive(Debug)]
#[derive(Clone)]
pub enum Command {
	Snd(String),
	Set(String, String),
	Add(String, String),
	Mul(String, String),
	Mod(String, String),
	Rcv(String),
	Jgz(String, String)
}

fn parse_line(s: &str) -> Command {
	let spl = s.split_whitespace().collect::<Vec<&str>>();
	match spl[0] {
		"snd" => { 
			Command::Snd(spl[1].to_string()) 
		},
		"set" => { 
			Command::Set(spl[1].to_string(), spl[2].to_string())
		},
		"add" => { 
			Command::Add(spl[1].to_string(), spl[2].to_string()) 
		},
		"mul" => {
			Command::Mul(spl[1].to_string(), spl[2].to_string())
		},
		"mod" => {
			Command::Mod(spl[1].to_string(), spl[2].to_string())
		},
		"rcv" => {
			Command::Rcv(spl[1].to_string())
		},
		"jgz" => {
			Command::Jgz(spl[1].to_string(), spl[2].to_string())
		},
		_ => {
			println!("ILLEGAL COMMAND");
			Command::Rcv("0".to_string())
		}
	}
}

fn parse_input(path: &str) -> Input {
	let f = File::open(path).unwrap();
	BufReader::new(f).lines().flatten().map(|s| parse_line(&s)).collect()
}

#[derive(Debug)]
pub struct Vm {
	pub regs: HashMap<String, i64>,
	pub incoming: VecDeque<i64>,
	pub outgoing: VecDeque<i64>,
	pub ip: i64,
	pub done: bool,
	pub commands: Input,
	pub locked: bool,
	pub sends: usize
}

impl Vm
{
    pub fn new(id: i64, cmds: &Input) -> Self {
		let mut regs:HashMap<String, i64> = HashMap::new();
		for r in 'a'..'z' {
			regs.insert(r.to_string(), 0);
		}
		regs.insert("p".to_string(), id);
		Self {
			regs: regs,
			incoming: VecDeque::new(),
			outgoing: VecDeque::new(),
			ip: 0,
			done: false,
			commands: cmds.to_vec(),
			locked: false,
			sends: 0
		}
	}

	pub fn tick(&mut self) {
		if self.ip < 0 || self.ip >= self.commands.len() as i64 {
			self.done = true;
			return;
		}
		self.locked = false;
		match &self.commands[self.ip as usize] {
			Command::Set(x, y) => {
				let val = match y.parse() { Ok(v) => v, Err(_) => self.regs[y] };
				self.regs.insert(x.to_string(), val);
				self.ip += 1;
			},
			Command::Mul(x, y) => {
				let val1 = self.regs[x];
				let val2 = match y.parse() { Ok(v) => v, Err(_) => self.regs[y] };
				self.regs.insert(x.to_string(), val1 * val2);
				self.ip += 1;
			},
			Command::Jgz(x, y) => {
				let val = match x.parse() { Ok(v) => v, Err(_) => self.regs[x] };
				if val > 0 {
					let offset =  match y.parse() { Ok(v) => v, Err(_) => self.regs[y] };
					self.ip += offset;
				} else {
					self.ip += 1;
				}
			}
			Command::Add(x, y) => {
				let val1 = self.regs[x];
				let val2 = match y.parse() { Ok(v) => v, Err(_) => self.regs[y] };
				self.regs.insert(x.to_string(), val1 + val2);
				self.ip += 1;
			},
			Command::Mod(x, y) => {
				let val1 = self.regs[x];
				let val2 = match y.parse() { Ok(v) => v, Err(_) => self.regs[y] };
				self.regs.insert(x.to_string(), val1 % val2);
				self.ip += 1;
			},
			Command::Snd(x) => {
				let val = match x.parse() { Ok(v) => v, Err(_) => self.regs[x] };
				self.outgoing.push_back(val);
				self.sends += 1;
				self.ip += 1;
			},
			Command::Rcv(x) => {
				if self.incoming.len() > 0 {
					self.regs.insert(x.to_string(), self.incoming.pop_front().unwrap());
					self.ip += 1;
				} else {
					self.locked = true;
				}
			},
			_ => {
				println!("Invalid Command: {:?}", self.commands[self.ip as usize]);
			}
		}
	}
}

fn part2(commands: &Input) -> usize {
	let mut vm0 = Vm::new(0, commands);
	let mut vm1 = Vm::new(1, commands);
	while !(vm0.done && vm1.done) && !(vm0.locked && vm1.locked) {
		vm0.tick();
		vm1.tick();
		while vm0.outgoing.len() != 0 {
			vm1.incoming.push_back(vm0.outgoing.pop_front().unwrap());
		}
		while vm1.outgoing.len() != 0 {
			vm0.incoming.push_back(vm1.outgoing.pop_front().unwrap());
		}
	}

	vm1.sends
}

fn part1(commands: &Input) -> i64 {
	let mut regs:HashMap<String, i64> = HashMap::new();
	for r in 'a'..'z' {
		regs.insert(r.to_string(), 0);
	}
	let mut freq = 0;
	let mut ip: i64 = 0;

	while ip >= 0 && ip < commands.len() as i64 {
		match &commands[ip as usize] {
			Command::Set(x, y) => {
				let val = match y.parse() { Ok(v) => v, Err(_) => regs[y] };
				regs.insert(x.to_string(), val);
				ip += 1;
			},
			Command::Mul(x, y) => {
				let val1 = regs[x];
				let val2 = match y.parse() { Ok(v) => v, Err(_) => regs[y] };
				regs.insert(x.to_string(), val1 * val2);
				ip += 1;
			},
			Command::Jgz(x, y) => {
				let val = match x.parse() { Ok(v) => v, Err(_) => regs[x] };
				if val > 0 {
					let offset =  match y.parse() { Ok(v) => v, Err(_) => regs[y] };
					ip += offset;
				} else {
					ip += 1;
				}
			}
			Command::Add(x, y) => {
				let val1 = regs[x];
				let val2 = match y.parse() { Ok(v) => v, Err(_) => regs[y] };
				regs.insert(x.to_string(), val1 + val2);
				ip += 1;
			},
			Command::Mod(x, y) => {
				let val1 = regs[x];
				let val2 = match y.parse() { Ok(v) => v, Err(_) => regs[y] };
				regs.insert(x.to_string(), val1 % val2);
				ip += 1;
			},
			Command::Snd(x) => {
				freq = match x.parse() { Ok(v) => v, Err(_) => regs[x] };
				ip += 1;
			},
			Command::Rcv(x) => {
				regs.insert(x.to_string(), freq);
				ip += 1;
				break;
			},
			_ => {
				println!("Invalid Command: {:?}", commands[ip as usize]);
				break;
			}
		}
	}

	freq
}

pub fn main() {
	let commands = parse_input("./input/day18/input.txt");

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