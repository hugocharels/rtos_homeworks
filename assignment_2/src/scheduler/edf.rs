use crate::{
	models::{Job, TaskSet, TimeStep},
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
	fn is_schedulable(&self, taskset: &mut TaskSet, cores: &u32) -> SchedulabilityResult {
		if taskset.system_utilization() > *cores as f64 || taskset.utilization_max() > 1.0 {
			return SchedulabilityResult::UnschedulableShortcut;
		}

		SchedulabilityResult::Unknown
	}
}

impl SchedulerSimulator for EDF {
	fn next_jobs<'a>(&'a self, queue: &'a mut Vec<Job>, cores: &u32) -> Vec<&'a mut Job> {
		// TODO
		Vec::new()
	}


	fn t_max(&self, taskset: &TaskSet) -> TimeStep {
		// TODO: [O_max, O_max + 2P)
		TimeStep::MAX
	}
}