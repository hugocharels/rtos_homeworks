use num_cpus;

mod taskset_parser;
mod arg_parser;


fn main() {
	let matches = arg_parser::get_arg_parser().get_matches();

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
}
