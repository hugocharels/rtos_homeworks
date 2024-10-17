use crate::models::TaskSet;
use super::strategy::SchedulerStrategy;


pub struct RoundRobin;

impl SchedulerStrategy for RoundRobin {
	fn is_schedulable(&self, task_set: &TaskSet) -> bool {
		// Round Robin-specific schedulability logic here
		true // placeholder
	}
}