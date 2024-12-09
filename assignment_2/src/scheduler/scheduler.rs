use crate::models::TaskSet;
use crate::scheduler::result::SchedulabilityResult;

pub trait Scheduler {
	fn is_schedulable(&mut self, task_set: &mut TaskSet, cores: usize) -> SchedulabilityResult;
}