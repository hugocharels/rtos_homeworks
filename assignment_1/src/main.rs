use crate::schedulers::{result::SchedulabilityResult, SchedulerStrategy};

mod arg_parser;
mod models;
mod schedulers;
mod taskset_parser;


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