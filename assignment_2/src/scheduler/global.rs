use crate::models::TaskSet;
use crate::scheduler::result::SchedulabilityResult;
use crate::scheduler::scheduler::Scheduler;

pub struct Global;

impl Global {}

impl Scheduler for Global {
	fn is_schedulable(&self, taskset: &mut TaskSet, cores: &u32) -> SchedulabilityResult {
		SchedulabilityResult::Unknown
	}
}

