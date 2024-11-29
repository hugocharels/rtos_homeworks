use clap::{Arg, Command};

fn validate_version(value: &str) -> Result<String, String> {
	match value {
		"global" | "partitioned" => Ok(value.to_string()),
		_ => {
			if value.parse::<usize>().is_ok() {
				Ok(value.to_string())
			} else {
				Err(format!("Invalid version: '{}'. Must be 'global', 'partitioned', or a natural number.", value))
			}
		}
	}
}

pub fn get_arg_parser() -> Command {
	Command::new("Scheduler")
		.disable_help_flag(true)
		.about("Task scheduler with various algorithms")
		.arg(
			Arg::new("taskset file")
				.help("The task set file to consider")
				.required(true)
				.index(1),
		)
		.arg(
			Arg::new("cores")
				.help("The number of cores to use (m)")
				.required(true)
				.index(2),
		)
		.arg(
			Arg::new("version")
				.short('v')
				.long("version")
				.help("The version of EDF to use (partitioned, global, or EDF(k))")
				.required(true)
				.value_name("VERSION")
				.value_parser(validate_version),
		)
		.arg(
			Arg::new("workers")
				.short('w')
				.long("workers")
				.help("The number of workers to run the simulation (default: number of cores)")
				.value_name("WORKERS"),
		)
		.arg(
			Arg::new("heuristic")
				.short('h')
				.long("heuristic")
				.help("The heuristic to use in case of partitioned scheduling (e.g., first fit, next fit, etc.)")
				.value_name("HEURISTIC")
				.value_parser(["ff", "nf", "bf", "wf"])
				.requires_if("partitioned", "version"),
		)
		.arg(
			Arg::new("ordering")
				.short('s')
				.long("sort")
				.help("The ordering of tasks by utilization (increasing or decreasing)")
				.value_name("ORDERING")
				.value_parser(["iu", "du"])
				.requires_if("partitioned", "version"),
		)
}


#[cfg(test)]
mod tests {
	use super::*;
	use clap::Command;

	fn get_command() -> Command {
		get_arg_parser()
	}

	#[test]
	fn parses_required_arguments() {
		let matches = get_command().try_get_matches_from(vec!["Scheduler", "taskset", "4", "-v", "global"]).unwrap();
		assert_eq!(matches.get_one::<String>("taskset file"), Some(&String::from("taskset")));
		assert_eq!(matches.get_one::<String>("cores"), Some(&String::from("4")));
		assert_eq!(matches.get_one::<String>("version"), Some(&String::from("global")));
	}

	#[test]
	fn parses_optional_workers_argument() {
		let matches = get_command().try_get_matches_from(vec!["Scheduler", "taskset", "4", "-v", "global", "-w", "8"]).unwrap();
		assert_eq!(matches.get_one::<String>("workers"), Some(&String::from("8")));
	}

	#[test]
	fn parses_optional_heuristic_argument() {
		let matches = get_command().try_get_matches_from(vec!["Scheduler", "taskset", "4", "-v", "partitioned", "-h", "ff"]).unwrap();
		assert_eq!(matches.get_one::<String>("heuristic"), Some(&String::from("ff")));
	}

	#[test]
	fn parses_optional_ordering_argument() {
		let matches = get_command().try_get_matches_from(vec!["Scheduler", "taskset", "4", "-v", "partitioned", "-s", "iu"]).unwrap();
		assert_eq!(matches.get_one::<String>("ordering"), Some(&String::from("iu")));
	}

	#[test]
	fn fails_without_required_arguments() {
		let result = get_command().try_get_matches_from(vec!["Scheduler"]);
		assert!(result.is_err());
	}

	#[test]
	fn fails_with_invalid_version() {
		let result = get_command().try_get_matches_from(vec!["Scheduler", "taskset", "4", "-v", "invalid"]);
		assert!(result.is_err());
	}

	#[test]
	fn fails_with_invalid_heuristic() {
		let result = get_command().try_get_matches_from(vec!["Scheduler", "taskset", "4", "-v", "partitioned", "-h", "invalid"]);
		assert!(result.is_err());
	}

	#[test]
	fn fails_with_invalid_ordering() {
		let result = get_command().try_get_matches_from(vec!["Scheduler", "taskset", "4", "-v", "partitioned", "-s", "invalid"]);
		assert!(result.is_err());
	}

	#[test]
	fn parses_all_arguments_correctly() {
		let matches = get_command().try_get_matches_from(vec![
			"Scheduler", "taskset", "4", "-v", "partitioned", "-w", "8", "-h", "bf", "-s", "du"
		]).unwrap();
		assert_eq!(matches.get_one::<String>("taskset file"), Some(&String::from("taskset")));
		assert_eq!(matches.get_one::<String>("cores"), Some(&String::from("4")));
		assert_eq!(matches.get_one::<String>("version"), Some(&String::from("partitioned")));
		assert_eq!(matches.get_one::<String>("workers"), Some(&String::from("8")));
		assert_eq!(matches.get_one::<String>("heuristic"), Some(&String::from("bf")));
		assert_eq!(matches.get_one::<String>("ordering"), Some(&String::from("du")));
	}

	#[test]
	fn workers_defaults_to_cores_if_not_provided() {
		let matches = get_command().try_get_matches_from(vec!["Scheduler", "taskset", "4", "-v", "global"]).unwrap();
		assert!(matches.get_one::<String>("workers").is_none());
	}
}
