use super::strategy::SchedulerStrategy;
use crate::models::TaskSet;
use crate::schedulers::result::SchedulabilityResult;


pub struct DM;

impl SchedulerStrategy for DM {
	fn is_schedulable(&self, task_set: &TaskSet) -> SchedulabilityResult {
		// DM-specific schedulability logic here
		SchedulabilityResult::Unknown
	}
}