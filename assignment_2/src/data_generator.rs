use crate::builder::Builder;
use crate::taskset_parser::read_taskset_from_file;
use rayon::ThreadPoolBuilder;

pub fn generate_data() {
	// Constants
	const TASKSETS_FOLDER: &str = "tasksets";
	const CORES: usize = 8;
	const VERSION: &str = "partitioned";
	const HEURISTIC: &str = "bf";
	const ORDERING: &str = "du";

	// Initialize CSV writer
	let mut writer = csv::Writer::from_path("results.csv").expect("Failed to open results.csv");
	writer.write_record(&["taskset", "workers", "result", "duration_us"]).expect("Failed to write CSV header");

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
				// Configure thread pool
				ThreadPoolBuilder::new()
					.num_threads(worker)
					.build()
					.expect("Failed to configure thread pool");

				// Read task set
				let mut task_set = read_taskset_from_file(taskset_file.clone());

				// Measure execution time
				let start = std::time::Instant::now();
				let result = scheduler.is_schedulable(&mut task_set, CORES);
				let duration = start.elapsed().as_micros();

				// Write results to CSV
				writer
					.write_record(&[taskset_file.clone(), worker.to_string(), (result as i32).to_string(), duration.to_string()])
					.expect("Failed to write CSV record");
				writer.flush().expect("Failed to flush CSV writer");

				// Log results
				println!(
					"Task set file: {}, workers: {}, result: {:?}, duration: {}µs",
					taskset_file, worker, result, duration
				);
			}
		}
	}
}
