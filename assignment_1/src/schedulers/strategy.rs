use crate::models::TaskSet;

pub trait SchedulerStrategy {
	fn is_schedulable(&self, task_set: &TaskSet) -> bool;
}
