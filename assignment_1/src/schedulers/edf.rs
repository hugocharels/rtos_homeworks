use crate::models::TaskSet;
use crate::schedulers::result::SchedulabilityResult;
use super::strategy::SchedulerStrategy;

pub struct EDF;

impl SchedulerStrategy for EDF {
	fn is_schedulable(&self, task_set: &TaskSet) -> SchedulabilityResult {
		// EDF-specific schedulability logic here
		SchedulabilityResult::Unknown
	}
}