use crate::schedulers::SchedulerStrategy;
use std::io::Write;

mod arg_parser;
mod models;
mod schedulers;
mod taskset_parser;

/*
fn main() {
	let matches = arg_parser::get_arg_parser().get_matches();
	let scheduling_algorithm = matches.get_one::<String>("scheduling algorithm").unwrap();
	let verbose = matches.get_flag("verbose");
	let taskset_file = matches.get_one::<String>("taskset file").unwrap();

	if verbose {
		println!("Scheduling algorithm: {}", scheduling_algorithm);
		println!("Task set file: {}", taskset_file);
	}

	// Read the task set from the file
	let taskset = taskset_parser::read_taskset_from_file(taskset_file);

	if verbose {
		println!("Task set: {:?}", taskset);
	}

	// Check if the task set is schedulable
	let mut context = schedulers::SchedulerContext::new(taskset);
	let strategy: Option<Box<dyn SchedulerStrategy>> = match scheduling_algorithm.as_str() {
		"dm" => Some(Box::new(schedulers::dm::DM)),
		"edf" => Some(Box::new(schedulers::edf::EDF)),
		"rr" => Some(Box::new(schedulers::round_robin::RoundRobin)),
		_ => None,
	};

	if let Some(strategy) = strategy {
		context.set_strategy(strategy.as_ref());
		let result = context.check_schedulability();
		if verbose {
			println!("Schedulability result: {:?}", result);
		}
		std::process::exit(result as i32);
	} else {
		println!("Invalid scheduling algorithm: {}", scheduling_algorithm);
		std::process::exit(SchedulabilityResult::Unknown as i32);
	}
}
*/
use std::fs::File;
///*
use std::{fs, io};

fn get_utilization_from_dir(dir: &str) -> &str {
	// Extract the percentage utilization from the directory name (e.g., "tasksets/10-tasks/30-percent")
	dir.split('/').last().unwrap_or("unknown").split('-').next().unwrap_or("unknown")
}

fn main() -> io::Result<()> {
	// Specify the directories for the task sets
	let taskset_dirs_80_percent = vec![
		"tasksets/80-percent/4-tasks",
		"tasksets/80-percent/6-tasks",
		"tasksets/80-percent/8-tasks",
		"tasksets/80-percent/10-tasks",
		"tasksets/80-percent/12-tasks",
		"tasksets/80-percent/14-tasks",
		"tasksets/80-percent/16-tasks",
		"tasksets/80-percent/18-tasks",
		"tasksets/80-percent/20-tasks",
	];
	let taskset_dirs_10_tasks = vec![
		"tasksets/10-tasks/10-percent",
		"tasksets/10-tasks/20-percent",
		"tasksets/10-tasks/30-percent",
		"tasksets/10-tasks/40-percent",
		"tasksets/10-tasks/50-percent",
		"tasksets/10-tasks/60-percent",
		"tasksets/10-tasks/70-percent",
		"tasksets/10-tasks/80-percent",
		"tasksets/10-tasks/90-percent",
		"tasksets/10-tasks/100-percent",
	];

	// Algorithms to test
	let algorithms = vec!["dm", "edf", "rr"];

	// Prepare the CSV file for output
	let mut file = File::create("../scheduling_results.csv")?;
	writeln!(file, "Taskset,Algorithm,Number of Tasks,Utilization,Schedulable")?;

	// Iterate over the task sets for the 80-percent folder
	for dir in taskset_dirs_80_percent.iter() {
		println!("Processing task set directory: {}", dir);
		let paths = fs::read_dir(dir)?;
		for path in paths {
			let path = path?.path();
			if path.is_file() {
				let taskset = taskset_parser::read_taskset_from_file(path.to_str().unwrap());

				for &algorithm in &algorithms {
					let mut context = schedulers::SchedulerContext::new(taskset.clone());
					let strategy: Option<Box<dyn SchedulerStrategy>> = match algorithm {
						"dm" => Some(Box::new(schedulers::dm::DM)),
						"edf" => Some(Box::new(schedulers::edf::EDF)),
						"rr" => Some(Box::new(schedulers::round_robin::RoundRobin)),
						_ => None,
					};

					if let Some(strategy) = strategy {
						context.set_strategy(strategy.as_ref());
						let result = context.check_schedulability();
						writeln!(
							file,
							"{},{},{},{},{}",
							dir,
							algorithm,
							taskset.len(),
							"80%", // Assuming all sets in this dir have 80% utilization
							result as i32
						)?;
					}
				}
			}
			file.flush()?;
		}
	}

	// Repeat the same for the 10-tasks folder with different utilization
	for dir in taskset_dirs_10_tasks.iter() {
		println!("Processing task set directory: {}", dir);
		let paths = fs::read_dir(dir)?;
		for path in paths {
			let path = path?.path();
			if path.is_file() {
				let taskset = taskset_parser::read_taskset_from_file(path.to_str().unwrap());

				for &algorithm in &algorithms {
					let mut context = schedulers::SchedulerContext::new(taskset.clone());
					let strategy: Option<Box<dyn SchedulerStrategy>> = match algorithm {
						"dm" => Some(Box::new(schedulers::dm::DM)),
						"edf" => Some(Box::new(schedulers::edf::EDF)),
						"rr" => Some(Box::new(schedulers::round_robin::RoundRobin)),
						_ => None,
					};

					if let Some(strategy) = strategy {
						context.set_strategy(strategy.as_ref());
						let result = context.check_schedulability();
						writeln!(
							file,
							"{},{},{},{},{}",
							dir,
							algorithm,
							taskset.len(),
							get_utilization_from_dir(dir), // Custom function to extract utilization from the dir name
							result as i32
						)?;
					}
				}
			}
			file.flush()?;
		}
	}

	Ok(())
}
//*/