use super::strategy::SchedulerStrategy;
use crate::{
	models::{Job, TaskSet, TimeStep},
	schedulers::errors::SchedulingError,
	schedulers::result::SchedulabilityResult,
	schedulers::simulator_strategy::SchedulerSimulatorStrategy,
};

pub struct RoundRobin;

impl SchedulerStrategy for RoundRobin {
	fn is_schedulable(&self, task_set: &mut TaskSet) -> SchedulabilityResult {
		if task_set.system_utilization() > 1.0 {
			return SchedulabilityResult::UnschedulableShortcut;
		}

		match self.simulate(task_set) {
			Ok(()) => SchedulabilityResult::SchedulableSimulated,
			Err(SchedulingError::DeadlineMiss { job: _job, t: _t }) => SchedulabilityResult::UnschedulableSimulated,
		}
	}
}

impl SchedulerSimulatorStrategy for RoundRobin {
	fn next_job<'a>(&'a self, queue: &'a mut Vec<Job>) -> Option<&'a mut Job> {
		if !queue.is_empty() {
			Some(&mut queue[0])
		} else {
			None
		}
	}

	fn t_max(&self, task_set: &TaskSet) -> TimeStep {
		task_set.hyperperiod()
	}
}