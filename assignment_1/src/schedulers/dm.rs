use super::strategy::SchedulerStrategy;
use crate::{
	models::{Job, TaskSet, TimeStep},
	schedulers::result::SchedulabilityResult,
};

pub struct DM;

impl SchedulerStrategy for DM {
	fn is_schedulable(&self, task_set: &mut TaskSet) -> SchedulabilityResult {
		if task_set.system_utilization() > 1.0 {
			return SchedulabilityResult::UnschedulableShortcut;
		} else if task_set.system_utilization() <= task_set.size() as f64 * (2f64.powf(1.0 / task_set.size() as f64) - 1.0) {
			return SchedulabilityResult::SchedulableShortcut;
		}

		match self.simulate(task_set) {
			Ok(_) => SchedulabilityResult::SchedulableSimulated,
			Err(_) => SchedulabilityResult::UnschedulableSimulated,
		};

		SchedulabilityResult::Unknown
	}

	fn next_job<'a>(&'a self, queue: &'a mut Vec<Job>) -> Option<&'a mut Job> {
		queue.iter_mut().reduce(|res, job| {
			if job.task().deadline() < res.task().deadline() {
				job
			} else {
				res
			}
		})
	}

	fn t_max(&self, task_set: &TaskSet) -> TimeStep {
		task_set.hyperperiod()
	}
}