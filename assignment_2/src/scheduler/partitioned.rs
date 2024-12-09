use crate::models::{TaskSet, TimeStep};
use crate::scheduler::{
	errors::SchedulingError,
	heuristics::strategy::HeuristicStrategy,
	orderings::strategy::OrderingStrategy,
	result::SchedulabilityResult,
	scheduler::Scheduler,
	simulator::SimpleMultiCoreSchedulerSimulator,
	simulator::SingleCorePartitionSchedulerSimulator,
};
use rayon::iter::{IntoParallelIterator, ParallelIterator};

pub struct Partitioned {
	heuristic: Option<Box<dyn HeuristicStrategy>>,
	ordering: Option<Box<dyn OrderingStrategy>>,
}

impl Partitioned {
	pub fn new() -> Partitioned {
		Self {
			heuristic: None,
			ordering: None,
		}
	}

	pub fn set_heuristic(&mut self, heuristic: Box<dyn HeuristicStrategy>) {
		self.heuristic = Some(heuristic);
	}

	pub fn set_ordering(&mut self, ordering: Box<dyn OrderingStrategy>) {
		self.ordering = Some(ordering);
	}
}

impl Scheduler for Partitioned {
	fn is_schedulable(&mut self, task_set: &mut TaskSet, cores: usize) -> SchedulabilityResult {
		if self.heuristic.is_none() || self.ordering.is_none() {
			return SchedulabilityResult::Unknown;
		} else if task_set.system_utilization() > cores as f64 || task_set.utilization_max() > 1.0 {
			return SchedulabilityResult::UnschedulableShortcut;
		} else if cores >= task_set.len() {
			return SchedulabilityResult::SchedulableShortcut;
		// TODO: Check if FF DU instead of false
		} else if false && task_set.system_utilization() <= (cores + 1) as f64 / 2f64 {
			return SchedulabilityResult::SchedulableShortcut;
		}

		// Simulate the scheduling
		match SimpleMultiCoreSchedulerSimulator::simulate(self, task_set, cores) {
			Ok(()) => SchedulabilityResult::SchedulableSimulated,
			Err(_) => SchedulabilityResult::UnschedulableSimulated,
		}
	}
}

impl SingleCorePartitionSchedulerSimulator for Partitioned {

	fn t_max(&self, task_set: &TaskSet) -> TimeStep {
		task_set.hyperperiod()
	}

	fn simulate(&self, task_set: &mut TaskSet) -> Result<(), SchedulingError> {
		let t_max = self.t_max(task_set);
		let mut task_id = 0;
		let mut queue = Vec::new();

		for t in 0..t_max {
			// Release jobs
			queue.extend(task_set.release_jobs(t));

			// Check for deadlines
			if let Some(missed_job) = queue.iter().find(|job| job.deadline_missed(t)) {
				return Err(SchedulingError::DeadlineMiss {
					job: missed_job.clone(),
					t,
				});
			}

			// Schedule the job with the right task index
			if let Some(job) = queue.iter_mut().find(|job| job.task().id() == task_id) {
				// Schedule the job
				job.schedule(1);

				// If the job is complete, remove it from the queue and go to the next job
				if job.is_complete() {
					task_id = (task_id + 1) % task_set.len() as u32;
				}
			}

			// Remove the completed jobs from the queue
			queue.retain(|job| !job.is_complete());
		}
		Ok(())
	}
}

impl SimpleMultiCoreSchedulerSimulator for Partitioned {
	fn simulate(&self, task_set: &mut TaskSet, cores: usize) -> Result<(), SchedulingError> {
		// Apply the ordering strategy
		self.ordering.as_ref().unwrap().apply_order(task_set);

		let splited_taskset: Vec<TaskSet> = self.heuristic
			.as_ref()
			.unwrap()
			.get_splited_taskset(task_set, cores)
			.map_err(|e| SchedulingError::PartitionedError(e))?;

		// Simulate the scheduling for each partition in parallel
		splited_taskset
			.into_par_iter()
			.try_for_each(|mut partition| {
				<Self as SingleCorePartitionSchedulerSimulator>::simulate(self, &mut partition)
			})
	}
}