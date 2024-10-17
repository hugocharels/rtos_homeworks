use crate::models::TaskSet;
use super::strategy::SchedulerStrategy;

pub struct EDF;

impl SchedulerStrategy for EDF {
	fn is_schedulable(&self, task_set: &TaskSet) -> bool {
		// EDF-specific schedulability logic here
		true // placeholder
	}
}