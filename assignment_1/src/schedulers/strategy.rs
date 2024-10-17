use crate::models::TaskSet;
use crate::schedulers::result::SchedulabilityResult;

pub trait SchedulerStrategy {
	fn is_schedulable(&self, task_set: &TaskSet) -> SchedulabilityResult;
}
