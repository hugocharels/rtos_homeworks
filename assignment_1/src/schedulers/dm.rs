use crate::models::TaskSet;
use super::strategy::SchedulerStrategy;


pub struct DM;

impl SchedulerStrategy for DM {
	fn is_schedulable(&self, task_set: &TaskSet) -> bool {
		// DM-specific schedulability logic here
		true // placeholder
	}
}