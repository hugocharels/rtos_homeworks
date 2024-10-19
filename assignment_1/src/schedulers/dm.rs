use super::strategy::SchedulerStrategy;
use crate::models::TaskSet;
use crate::schedulers::result::SchedulabilityResult;


pub struct DM;

impl SchedulerStrategy for DM {
	fn is_schedulable(&self, task_set: &TaskSet) -> SchedulabilityResult {
		if task_set.system_utilization() > 1.0 {
			return SchedulabilityResult::UnschedulableShortcut;
		} else if task_set.system_utilization() <= task_set.size() as f64 * (2f64.powf(1.0 / task_set.size() as f64) - 1.0) {
			return SchedulabilityResult::SchedulableShortcut;
		}

		// TODO: Simulate feasibility interval

		SchedulabilityResult::Unknown
	}
}