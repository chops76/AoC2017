use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;

type Input = Vec<Particle>;

#[derive(Debug)]
#[derive(Clone)]
struct Particle {
	pos: (i64, i64, i64),
	vel: (i64, i64, i64),
	acc: (i64, i64, i64),
	valid: bool
}

fn parse_line(s: &str) -> Particle {
	let spl = s.split(",").collect::<Vec<&str>>();
	Particle {
		pos: (spl[0][3..].parse().unwrap(), spl[1].parse().unwrap(), spl[2][..spl[2].len()-1].parse().unwrap()),
		vel: (spl[3][4..].parse().unwrap(), spl[4].parse().unwrap(), spl[5][..spl[5].len()-1].parse().unwrap()),
		acc: (spl[6][4..].parse().unwrap(), spl[7].parse().unwrap(), spl[8][..spl[8].len()-1].parse().unwrap()),
		valid: true
	}
}

fn parse_input(path: &str) -> Input {
	let f = File::open(path).unwrap();
	BufReader::new(f).lines().flatten().map(|s| parse_line(&s)).collect()
}

fn part1(particles: &Input) -> usize {
	let mut found = 0;
	let mut found_acc = 99999;
	let mut found_vel = 99999;
	let mut found_pos = 99999;
	for i in 0..particles.len() {
		let acc = particles[i].acc.0.abs() + particles[i].acc.1.abs() + particles[i].acc.2.abs();
		let vel = particles[i].vel.0.abs() + particles[i].vel.1.abs() + particles[i].vel.2.abs();
		let pos = particles[i].pos.0.abs() + particles[i].pos.1.abs() + particles[i].pos.2.abs();
		if acc < found_acc {
			found = i;
			found_acc = acc;
			found_vel = vel;
			found_pos = pos;
		} else if acc == found_acc {
			if vel < found_vel {
				found = i;
				found_acc = acc;
				found_vel = vel;
				found_pos = pos;
			} // This is as much as I needed
		}
	}

	found
}

fn part2(particles: &Input) -> usize {
	let mut part = particles.clone();
	for _ in 0..1000 {
		for i in 0..part.len() {
			if !part[i].valid {
				continue;
			}
			part[i].vel.0 += part[i].acc.0;
			part[i].vel.1 += part[i].acc.1;
			part[i].vel.2 += part[i].acc.2;
			part[i].pos.0 += part[i].vel.0;
			part[i].pos.1 += part[i].vel.1;
			part[i].pos.2 += part[i].vel.2;
		}
		for i in 0..part.len() - 1 {
			if !part[i].valid {
				continue;
			}
			for j in i+1..part.len() {
				if part[j].valid && part[i].pos == part[j].pos {
					part[i].valid = false;
					part[j].valid = false;
				}
			}
		}
	}
	part.iter().filter(|p| p.valid).count()
}

pub fn main() {
	let particles = parse_input("./input/day20/input.txt");

	let p1_timer = Instant::now();
    let p1_result = part1(&particles);
    let p1_time = p1_timer.elapsed();
	println!("Part 1: {}", p1_result);
	println!("Part 1 Time: {:?}", p1_time);

	let p2_timer = Instant::now();
    let p2_result = part2(&particles);
    let p2_time = p2_timer.elapsed();
	println!("Part 2: {}", p2_result);
	println!("Part 2 Time: {:?}", p2_time);
}