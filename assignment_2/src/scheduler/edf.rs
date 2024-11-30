use crate::{
	models::{Job, TaskSet},
	scheduler::result::SchedulabilityResult,
	scheduler::scheduler::Scheduler,
	scheduler::simulator::SchedulerSimulator,
};

pub struct EDF {
	k: u32,
}

impl EDF {
	pub fn new(k: u32) -> Self {
		Self { k }
	}
}

impl Scheduler for EDF {
	fn is_schedulable(&self, taskset: &mut TaskSet, cores: usize) -> SchedulabilityResult {
		if taskset.system_utilization() > cores as f64 || taskset.utilization_max() > 1.0 {
			return SchedulabilityResult::UnschedulableShortcut;
		}

		SchedulabilityResult::Unknown
	}
}

impl SchedulerSimulator for EDF {
	fn next_jobs<'a>(&'a self, queue: &'a mut Vec<Job>, cores: usize) -> Vec<&'a mut Job> {
		// TODO
		Vec::new()
	}
}