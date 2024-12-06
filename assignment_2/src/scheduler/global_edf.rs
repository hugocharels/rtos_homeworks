use crate::models::{Job, TaskSet};
use crate::scheduler::result::SchedulabilityResult;
use crate::scheduler::scheduler::Scheduler;
use crate::scheduler::simulator::SchedulerSimulator;
use rayon::prelude::ParallelSliceMut;

pub struct GlobalEDF;

impl GlobalEDF {}

impl Scheduler for GlobalEDF {
	fn is_schedulable(&self, taskset: &mut TaskSet, cores: usize) -> SchedulabilityResult {
		if taskset.system_utilization() > cores as f64 || taskset.utilization_max() > 1.0 {
			return SchedulabilityResult::UnschedulableShortcut;
		} else if cores >= taskset.len() {
			return SchedulabilityResult::SchedulableShortcut;
		} else if taskset.is_implicit_deadline() && taskset.system_utilization() <= cores as f64 - (cores - 1) as f64 * taskset.utilization_max() {
			return SchedulabilityResult::SchedulableShortcut;
		}

		match self.simulate(taskset, cores) {
			Ok(()) => SchedulabilityResult::SchedulableSimulated,
			Err(_) => SchedulabilityResult::UnschedulableSimulated,
		}
	}
}


impl SchedulerSimulator for GlobalEDF {
	fn next_jobs<'a>(&'a self, queue: &'a mut Vec<Job>, cores: usize) -> Vec<&'a mut Job> {
		// Sort the queue by deadline
		queue.par_sort_by_key(|job| job.deadline());

		// Return the "cores" first jobs
		queue.iter_mut().take(cores).collect()
	}
}