use super::strategy::SchedulerStrategy;
use crate::models::TaskSet;
use crate::schedulers::result::SchedulabilityResult;


pub struct RoundRobin;

impl SchedulerStrategy for RoundRobin {
	fn is_schedulable(&self, task_set: &TaskSet) -> SchedulabilityResult {
		if task_set.system_utilization() > 1.0 {
			return SchedulabilityResult::UnschedulableShortcut;
		}

		// TODO: Simulate feasibility interval

		SchedulabilityResult::Unknown
	}
}