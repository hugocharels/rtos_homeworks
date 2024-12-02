use crate::{
	models::{Job, TaskSet},
	scheduler::result::SchedulabilityResult,
	scheduler::scheduler::Scheduler,
	scheduler::simulator::SchedulerSimulator,
};
use rayon::prelude::ParallelSliceMut;

pub struct EDF {
	k: usize,
}

impl EDF {
	pub fn new(k: usize) -> Self {
		Self { k }
	}

	fn set_k_highest_priorities(&self, taskset: &mut TaskSet) {
		taskset.sort_by_deadline();
		for i in 0..self.k.min(taskset.len()) {
			taskset.set_highest_priority_on_task(i, true);
		}
	}

	fn get_priority(&self, job: &Job) -> u32 {
		if job.task().is_highest_priority() {
			0
		} else {
			job.deadline()
		}
	}
}

impl Scheduler for EDF {
	fn is_schedulable(&self, taskset: &mut TaskSet, cores: usize) -> SchedulabilityResult {
		if taskset.system_utilization() > cores as f64 || taskset.utilization_max() > 1.0 {
			return SchedulabilityResult::UnschedulableShortcut;
		}

		// Save the first k tasks so that they have the highest priority
		self.set_k_highest_priorities(taskset);

		match self.simulate(taskset, cores) {
			Ok(()) => SchedulabilityResult::SchedulableSimulated,
			Err(_) => SchedulabilityResult::UnschedulableSimulated,
		}
	}
}

impl SchedulerSimulator for EDF {
	fn next_jobs<'a>(&'a self, queue: &'a mut Vec<Job>, cores: usize) -> Vec<&'a mut Job> {
		// Sort the queue by deadline
		queue.par_sort_by_key(|job| self.get_priority(job));

		// Return the "cores" first jobs
		queue.iter_mut().take(cores).collect()
	}
}