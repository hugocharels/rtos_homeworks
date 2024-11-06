use super::strategy::SchedulerStrategy;
use crate::{
	models::{Job, TaskSet, TimeStep},
	schedulers::errors::SchedulingError,
	schedulers::result::SchedulabilityResult,
	schedulers::simulator_strategy::SchedulerSimulatorStrategy,
};
use rayon::prelude::*;

pub struct EDF;

impl SchedulerStrategy for EDF {
	fn is_schedulable(&self, task_set: &mut TaskSet) -> SchedulabilityResult {
		if task_set.system_utilization() > 1.0 {
			return SchedulabilityResult::UnschedulableShortcut;
		}

		if task_set.is_implicit_deadline() {
			return SchedulabilityResult::SchedulableShortcut;
		}

		match self.simulate(task_set) {
			Ok(()) => SchedulabilityResult::SchedulableSimulated,
			Err(SchedulingError::DeadlineMiss { job: _job, t: _t }) => SchedulabilityResult::UnschedulableSimulated,
			Err(_) => SchedulabilityResult::Unknown,
		}
	}
}

impl SchedulerSimulatorStrategy for EDF {
	fn next_job<'a>(&'a self, queue: &'a mut Vec<Job>) -> Option<&'a mut Job> {
		queue.par_iter_mut().min_by_key(|j| j.deadline())
	}

	fn t_max(&self, task_set: &TaskSet) -> TimeStep {
		task_set.tasks().iter().map(|t| t.deadline()).max().unwrap_or(0)
	}
}