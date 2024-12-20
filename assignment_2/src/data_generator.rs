use crate::{
	builder::Builder,
	scheduler::result::SchedulabilityResult,
	taskset_parser::read_taskset_from_file,
};
use rayon::ThreadPoolBuilder;

#[allow(dead_code)]
pub fn generate_time_paritioned_bfdu_worker() {
	// Constants
	const TASKSETS_FOLDER: &str = "tasksets";
	const CORES: usize = 8;
	const VERSION: &str = "partitioned";
	const HEURISTIC: &str = "bf";
	const ORDERING: &str = "du";
	const RUNS: usize = 100;

	// Initialize CSV writer
	let mut writer = csv::Writer::from_path("results_partitioned_bfdu_time_workers.csv").expect("Failed to open results_partitioned_bfdu_time_workers.csv");
	writer.write_record(&["taskset", "workers", "result", "duration"]).expect("Failed to write CSV header");

	// Create scheduler
	if let Some(scheduler) = Builder::new()
		.set_version(&VERSION.to_string())
		.set_heuristic(Some(&HEURISTIC.to_string()))
		.set_ordering(Some(&ORDERING.to_string()))
		.build()
	{
		for taskset_entry in std::fs::read_dir(TASKSETS_FOLDER).expect("Failed to read tasksets folder") {
			let taskset_path = taskset_entry.expect("Failed to read taskset entry").path();
			let taskset_file = taskset_path.display().to_string();

			for worker in 1..=32 {
				// Create a custom thread pool
				let pool = ThreadPoolBuilder::new()
					.num_threads(worker)
					.build()
					.expect("Failed to configure thread pool");

				// Initialize accumulators for results and durations
				let mut result = SchedulabilityResult::Unknown;
				let mut total_duration = 0;

				// Run the computation multiple times
				for _ in 0..RUNS {
					pool.install(|| {
						// Read task set
						let mut task_set = read_taskset_from_file(taskset_file.clone());

						// Measure execution time
						let start = std::time::Instant::now();
						result = scheduler.is_schedulable(&mut task_set, CORES);
						let duration = start.elapsed().as_micros();

						// Accumulate results and durations
						total_duration += duration;
					});
				}

				// Calculate means
				let mean_duration = total_duration as f64 / RUNS as f64;

				// Write mean results to CSV
				writer
					.write_record(&[
						taskset_file.clone(),
						worker.to_string(),
						format!("{:?}", result as i32),
						format!("{:.2}", mean_duration),
					])
					.expect("Failed to write CSV record");

				// Log mean results
				// println!(
				// 	"Task set file: {}, workers: {}, mean result: {:?}, mean duration: {:.2}µs",
				// 	taskset_file, worker, result as i32, mean_duration
				// );
			}
			writer.flush().expect("Failed to flush CSV writer");
		}
	}
}

#[allow(dead_code)]
pub fn generate_result_partitioned() {
	const TASKSETS_FOLDER: &str = "tasksets";
	const CORES: usize = 8;
	const VERSION: &str = "partitioned";
	const HEURISTICS: [&str; 4] = ["bf", "ff", "nf", "wf"];
	const ORDERINGS: [&str; 2] = ["du", "iu"];

	let mut writer = csv::Writer::from_path("results_partitioned.csv").expect("Failed to open results_partitioned.csv");
	writer.write_record(&["taskset", "heuristic", "ordering", "result"]).expect("Failed to write CSV header");


	for taskset_entry in std::fs::read_dir(TASKSETS_FOLDER).expect("Failed to read tasksets folder") {
		let taskset_path = taskset_entry.expect("Failed to read taskset entry").path();
		let taskset_file = taskset_path.display().to_string();
		let mut task_set = read_taskset_from_file(taskset_file.clone());

		for heuristic in HEURISTICS.iter() {
			for ordering in ORDERINGS.iter() {
				if let Some(scheduler) = Builder::new()
					.set_version(&VERSION.to_string())
					.set_heuristic(Some(&heuristic.to_string()))
					.set_ordering(Some(&ordering.to_string()))
					.build()
				{
					let result = scheduler.is_schedulable(&mut task_set, CORES);
					writer
						.write_record(&[taskset_file.clone(), heuristic.to_string(), ordering.to_string(), format!("{:?}", result as i32)])
						.expect("Failed to write CSV record");

					println!(
						"Task set file: {}, heuristic: {}, ordering: {}, result: {:?}",
						taskset_file, heuristic, ordering, result as i32
					);
				}
			}
		}
		writer.flush().expect("Failed to flush CSV writer");
	}
}

#[allow(dead_code)]
pub fn generate_result_edf() {
	// Constants
	const TASKSETS_FOLDER: &str = "tasksets";
	const CORES: usize = 8;
	const VERSIONS: [&str; 9] = ["global", "0", "1", "2", "3", "4", "5", "6", "7"];

	// Initialize CSV writer
	let mut writer = csv::Writer::from_path("results_result_edf.csv").expect("Failed to open results_result_edf.csv");
	writer.write_record(&["taskset", "version", "result"]).expect("Failed to write CSV header");

	for taskset_entry in std::fs::read_dir(TASKSETS_FOLDER).expect("Failed to read tasksets folder") {
		let taskset_path = taskset_entry.expect("Failed to read taskset entry").path();
		let taskset_file = taskset_path.display().to_string();
		let mut task_set = read_taskset_from_file(taskset_file.clone());

		for version in VERSIONS.iter() {
			if let Some(scheduler) = Builder::new()
				.set_version(&version.to_string())
				.build()
			{
				let result = scheduler.is_schedulable(&mut task_set, CORES);
				writer
					.write_record(&[taskset_file.clone(), version.parse().unwrap(), format!("{:?}", result as i32)])
					.expect("Failed to write CSV record");

				println!(
					"Task set file: {}, version: {}, result: {:?}",
					taskset_file, version, result as i32
				);
			}
		}
		writer.flush().expect("Failed to flush CSV writer");
	}
}