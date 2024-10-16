use clap::{Arg, Command};


pub(crate) fn get_parser() -> Command {
	Command::new("Scheduler")
		.about("Task scheduler with various algorithms")
		.arg(
			Arg::new("scheduling algorithm")
				.help("Specify the scheduling algorithm: audsley, edf, or rr")
				.required(true)
				.index(1)
				.value_parser(["audsley", "edf", "rr"]),
		)
		.arg(
			Arg::new("verbose")
				.short('v')
				.help("Enable verbose output")
				.action(clap::ArgAction::SetTrue),
		)
		.arg(
			Arg::new("taskset file")
				.help("Specify the task set file")
				.required(true)
				.index(2),
		)
}