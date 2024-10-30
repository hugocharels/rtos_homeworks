use crate::models::{Task, TaskSet, TimeStep};
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn read_taskset_from_file(file_path: &str) -> TaskSet {
	let file = File::open(file_path).expect("Unable to open file");
	let reader = BufReader::new(file);

	let tasks: Vec<Task> = reader.lines()
		.enumerate()
		.map(|(id, line)| {
			let line = line.expect("Unable to read line");
			let values: Vec<TimeStep> = line
				.split(',')
				.map(|s| s.trim().parse().expect("Invalid number"))
				.collect();
			if values.len() != 4 {
				panic!("Each line must have exactly 4 values: offset, wcet, deadline, period");
			}
			Task::new(id as u32, values[0], values[1], values[2], values[3])
		})
		.collect();

	TaskSet::new(tasks)
}
