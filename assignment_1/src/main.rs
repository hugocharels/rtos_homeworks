mod parser;
mod models;

fn main() {
	let matches = parser::get_parser().get_matches();
	let scheduling_algorithm = matches.get_one::<String>("scheduling algorithm").unwrap();
	let verbose = matches.get_flag("verbose");
	let taskset_file = matches.get_one::<String>("taskset file").unwrap();

	println!("Algorithm: {}", scheduling_algorithm);
	if verbose {
		println!("Verbose mode is enabled");
	}
	println!("Task set file: {}", taskset_file);
}