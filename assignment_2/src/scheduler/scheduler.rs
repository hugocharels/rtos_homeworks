use crate::models::TaskSet;
use crate::scheduler::result::SchedulabilityResult;

pub trait Scheduler {
	fn is_schedulable(&self, taskset: &mut TaskSet, cores: usize) -> SchedulabilityResult;
}