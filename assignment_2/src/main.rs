use num_cpus;
use rayon::ThreadPoolBuilder;
mod taskset_parser;
mod arg_parser;
mod models;
mod builder;
mod scheduler;
use arg_parser::get_arg_parser;
use builder::Builder;
use taskset_parser::read_taskset_from_file;

fn main() {
	let matches = get_arg_parser().get_matches();

	let taskset_file = matches.get_one::<String>("taskset file").expect("Task set file is required");
	let cores = matches
		.get_one::<String>("cores")
		.expect("Number of cores is required")
		.parse::<u32>()
		.expect("Invalid number for cores");

	let version = matches.get_one::<String>("version").expect("Version is required");

	// Default workers to the number of logical cores if not provided
	let workers = matches
		.get_one::<String>("workers")
		.map(|w| w.parse::<u32>().expect("Invalid number for workers"))
		.unwrap_or_else(|| num_cpus::get() as u32);

	let heuristic = matches.get_one::<String>("heuristic");
	let ordering = matches.get_one::<String>("ordering");

	println!(
		"Task set file: {}, cores: {}, version: {}, workers: {}, heuristic: {:?}, ordering: {:?}",
		taskset_file, cores, version, workers, heuristic, ordering
	);

	// Set the global thread pool to use a specific number of threads
	ThreadPoolBuilder::new()
		.num_threads(workers as usize)
		.build_global()
		.expect("Failed to set up the global thread pool");

	// Read the task set from the file
	let taskset = read_taskset_from_file(taskset_file);

	// Create the scheduler
	let scheduler = Builder::new()
		.set_version(version)
		.set_heuristic(heuristic)
		.set_ordering(ordering)
		.set_workers(workers)
		.build();

	// Schedule the task set
	// let result = scheduler.check_schedulability(taskset, cores);
	// std::process::exit(result as i32);
}
