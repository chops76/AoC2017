use std::time::Instant;
use std::collections::HashMap;

fn part1() -> usize {
	let mut tape: HashMap<i32, bool> = HashMap::new();
	let mut pos = 0;
	let mut state = 'A';
	for _ in 0..12459852 {
		if !tape.contains_key(&pos) {
			tape.insert(pos, false);
		}
		let cur_val = tape[&pos];
		match state {
			'A' => {
				if !cur_val {
					tape.insert(pos, true);
					pos += 1;
					state = 'B';
				} else {
					tape.insert(pos, true);
					pos -= 1;
					state = 'E';
				}
			},
			'B' => {
				if !cur_val {
					tape.insert(pos, true);
					pos += 1;
					state = 'C';
				} else {
					tape.insert(pos, true);
					pos += 1;
					state = 'F';
				}
			},
			'C' => {
				if !cur_val {
					tape.insert(pos, true);
					pos -= 1;
					state = 'D';
				} else {
					tape.insert(pos, false);
					pos += 1;
					state = 'B';
				}
			},
			'D' => {
				if !cur_val {
					tape.insert(pos, true);
					pos += 1;
					state = 'E';
				} else {
					tape.insert(pos, false);
					pos -= 1;
					state = 'C';
				}
			},
			'E' => {
				if !cur_val {
					tape.insert(pos, true);
					pos -= 1;
					state = 'A';
				} else {
					tape.insert(pos, false);
					pos += 1;
					state = 'D';
				}
			},
			'F' => {
				if !cur_val {
					tape.insert(pos, true);
					pos += 1;
					state = 'A';
				} else {
					tape.insert(pos, true);
					pos += 1;
					state = 'C';
				}
			},
			_ => println!("ILLEGAL STATE")
		}
	}

	tape.iter().filter(|(_,v)| **v).count()
}

pub fn main() {
	let p1_timer = Instant::now();
    let p1_result = part1();
    let p1_time = p1_timer.elapsed();
	println!("Part 1: {}", p1_result);
	println!("Part 1 Time: {:?}", p1_time);
}