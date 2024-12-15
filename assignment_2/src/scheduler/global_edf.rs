use crate::models::{Job, TaskSet};
use crate::scheduler::{
	errors::SchedulingError,
	result::SchedulabilityResult,
	scheduler::Scheduler,
	simulator::{MultiCoreSchedulerSimulator, SimpleMultiCoreSchedulerSimulator},
};

pub struct GlobalEDF;

impl GlobalEDF {}

impl Scheduler for GlobalEDF {
	fn is_schedulable(&self, task_set: &mut TaskSet, cores: usize) -> SchedulabilityResult {
		if task_set.system_utilization() > cores as f64 || task_set.utilization_max() > 1.0 {
			return SchedulabilityResult::UnschedulableShortcut;
		} else if cores >= task_set.len() {
			return SchedulabilityResult::SchedulableShortcut;
		} else if task_set.is_implicit_deadline() && task_set.system_utilization() <= cores as f64 - (cores - 1) as f64 * task_set.utilization_max() {
			return SchedulabilityResult::SchedulableShortcut;
		}

		match MultiCoreSchedulerSimulator::simulate(self, task_set, cores) {
			Ok(()) => SchedulabilityResult::Unknown,
			Err(_) => SchedulabilityResult::UnschedulableSimulated,
		}
	}
}

impl SimpleMultiCoreSchedulerSimulator for GlobalEDF {
	fn simulate(&self, task_set: &mut TaskSet, cores: usize) -> Result<(), SchedulingError> {
		<Self as MultiCoreSchedulerSimulator>::simulate(self, task_set, cores)
	}
}

impl MultiCoreSchedulerSimulator for GlobalEDF {
	fn next_jobs<'a>(&'a self, queue: &'a mut Vec<Job>, cores: usize) -> Vec<&'a mut Job> {
		// Sort the queue by deadline
		queue.sort_by_key(|job| job.deadline());

		// Return the "cores" first jobs
		queue.iter_mut().take(cores).collect()
	}
}