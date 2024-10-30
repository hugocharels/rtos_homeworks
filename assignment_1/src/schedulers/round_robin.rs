use super::strategy::SchedulerStrategy;
use crate::{
	models::{Job, TaskSet, TimeStep},
	schedulers::result::SchedulabilityResult,
};


pub struct RoundRobin;

impl SchedulerStrategy for RoundRobin {
	fn is_schedulable(&self, task_set: &mut TaskSet) -> SchedulabilityResult {
		if task_set.system_utilization() > 1.0 {
			return SchedulabilityResult::UnschedulableShortcut;
		}

		match self.simulate(task_set) {
			Ok(_) => SchedulabilityResult::SchedulableSimulated,
			Err(_) => SchedulabilityResult::UnschedulableSimulated,
		};

		SchedulabilityResult::Unknown
	}

	fn next_job<'a>(&'a self, queue: &'a mut Vec<Job>) -> Option<&'a mut Job> {
		queue.is_empty().then(|| &mut queue[0])
	}

	fn t_max(&self, task_set: &TaskSet) -> TimeStep {
		// calculate the least common multiple of all periods
		todo!()
	}
}