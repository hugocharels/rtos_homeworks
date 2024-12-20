use num_cpus;
use rayon::ThreadPoolBuilder;
mod taskset_parser;
mod arg_parser;
mod models;
mod builder;
mod scheduler;
mod data_generator;

use arg_parser::get_arg_parser;
use builder::Builder;
use taskset_parser::read_taskset_from_file;
// use crate::data_generator::*; // If you want to generate some data for plots

fn main() {
	let matches = get_arg_parser().get_matches();

	let taskset_file = matches.get_one::<String>("taskset file").expect("Task set file is required");
	let cores = matches
		.get_one::<String>("cores")
		.expect("Number of cores is required")
		.parse::<usize>()
		.expect("Invalid number for cores");

	let version = matches.get_one::<String>("version").expect("Version is required");

	// Default workers to the number of logical cores if not provided
	let workers = matches
		.get_one::<String>("workers")
		.map(|w| w.parse::<usize>().expect("Invalid number for workers"))
		.unwrap_or_else(|| num_cpus::get());

	let heuristic = matches.get_one::<String>("heuristic");
	let ordering = matches.get_one::<String>("ordering");

	println!(
		"Task set file: {}, cores: {}, version: {}, workers: {}, heuristic: {:?}, ordering: {:?}",
		taskset_file, cores, version, workers, heuristic, ordering
	);

	// Set the global thread pool to use a specific number of threads
	ThreadPoolBuilder::new()
		.num_threads(workers)
		.build_global()
		.expect("Failed to set up the global thread pool");

	// Read the task set from the file
	let mut task_set = read_taskset_from_file(taskset_file.to_string());

	// Create the scheduler
	let scheduler = Builder::new()
		.set_version(version)
		.set_heuristic(heuristic)
		.set_ordering(ordering)
		.build();

	if !scheduler.is_none() {
		// Schedule the task set
		let result = scheduler.unwrap().is_schedulable(&mut task_set, cores);
		std::process::exit(result as i32);
	}
}
