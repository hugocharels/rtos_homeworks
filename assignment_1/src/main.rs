mod arg_parser;
mod models;
mod schedulers;
mod taskset_parser;

fn main() {
	let matches = arg_parser::get_arg_parser().get_matches();
	let scheduling_algorithm = matches.get_one::<String>("scheduling algorithm").unwrap();
	let verbose = matches.get_flag("verbose");
	let taskset_file = matches.get_one::<String>("taskset file").unwrap();

	println!("Algorithm: {}", scheduling_algorithm);
	if verbose {
		println!("Verbose mode is enabled");
	}
	println!("Task set file: {}", taskset_file);

	// Read the task set from the file
	let taskset = taskset_parser::read_taskset_from_file(taskset_file);
	println!("Task set loaded: {:?}", taskset);
}