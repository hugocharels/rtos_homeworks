use super::strategy::SchedulerStrategy;
use crate::models::TaskSet;
use crate::schedulers::result::SchedulabilityResult;


pub struct RoundRobin;

impl SchedulerStrategy for RoundRobin {
	fn is_schedulable(&self, task_set: &TaskSet) -> SchedulabilityResult {
		// Round Robin-specific schedulability logic here
		SchedulabilityResult::Unknown
	}
}