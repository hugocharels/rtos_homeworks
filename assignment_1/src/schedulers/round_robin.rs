use super::strategy::SchedulerStrategy;
use crate::{
	models::{Job, TaskSet, TimeStep},
	schedulers::errors::SchedulingError,
	schedulers::result::SchedulabilityResult,
};
use num::Integer;


pub struct RoundRobin;

impl SchedulerStrategy for RoundRobin {
	fn is_schedulable(&self, task_set: &mut TaskSet) -> SchedulabilityResult {
		if task_set.system_utilization() > 1.0 {
			return SchedulabilityResult::UnschedulableShortcut;
		}

		match self.simulate(task_set) {
			Ok(()) => SchedulabilityResult::SchedulableSimulated,
			Err(SchedulingError::DeadlineMiss { job: _job, t: _t }) => SchedulabilityResult::UnschedulableSimulated,
			Err(_) => SchedulabilityResult::Unknown,
		}
	}

	fn next_job<'a>(&'a self, queue: &'a mut Vec<Job>) -> Option<&'a mut Job> {
		queue.is_empty().then(|| &mut queue[0])
	}

	fn t_max(&self, task_set: &TaskSet) -> TimeStep {
		// LCM of all periods
		task_set.tasks().iter().fold(1, |acc, t| acc.lcm(&t.period()))
	}
}