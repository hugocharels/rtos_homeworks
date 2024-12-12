use crate::builder::Builder;
use crate::scheduler::result::SchedulabilityResult;
use crate::taskset_parser::read_taskset_from_file;
use rayon::ThreadPoolBuilder;

pub fn generate_data() {
	// Constants
	const TASKSETS_FOLDER: &str = "tasksets";
	const CORES: usize = 8;
	const VERSION: &str = "partitioned";
	const HEURISTIC: &str = "bf";
	const ORDERING: &str = "du";
	const RUNS: usize = 100;

	// Initialize CSV writer
	let mut writer = csv::Writer::from_path("results.csv").expect("Failed to open results.csv");
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
				writer.flush().expect("Failed to flush CSV writer");

				// Log mean results
				println!(
					"Task set file: {}, workers: {}, mean result: {:?}, mean duration: {:.2}µs",
					taskset_file, worker, result as i32, mean_duration
				);
			}
		}
	}
}
