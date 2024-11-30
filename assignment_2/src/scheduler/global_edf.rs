use crate::models::{Job, TaskSet, TimeStep};
use crate::scheduler::result::SchedulabilityResult;
use crate::scheduler::scheduler::Scheduler;
use crate::scheduler::simulator::SchedulerSimulator;

pub struct GlobalEDF;

impl GlobalEDF {}

impl Scheduler for GlobalEDF {
	fn is_schedulable(&self, taskset: &mut TaskSet, cores: &u32) -> SchedulabilityResult {
		if taskset.system_utilization() > *cores as f64 || taskset.utilization_max() > 1.0 {
			return SchedulabilityResult::UnschedulableShortcut;
		}

		match self.simulate(taskset, cores) {
			Ok(()) => SchedulabilityResult::SchedulableSimulated,
			Err(_) => SchedulabilityResult::UnschedulableSimulated,
		}
	}
}


impl SchedulerSimulator for GlobalEDF {
	fn next_jobs<'a>(&'a self, queue: &'a mut Vec<Job>, cores: &u32) -> Vec<&'a mut Job> {
		// Sort the queue by deadline
		queue.sort_by_key(|job| job.deadline());

		// Select the first `cores` jobs that are not scheduled
		let selected_jobs: Vec<_> = queue.iter_mut()
			.filter(|job| !job.is_scheduled())
			.take(*cores as usize)
			.collect();

		selected_jobs
	}

	fn t_max(&self, taskset: &TaskSet) -> TimeStep {
		// TODO: [O_max, O_max + 2P)
		TimeStep::MAX
	}
}