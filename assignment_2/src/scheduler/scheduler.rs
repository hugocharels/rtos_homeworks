use crate::{
	models::TaskSet,
	scheduler::result::SchedulabilityResult,
};

pub trait Scheduler: Send + Sync {
	fn is_schedulable(&self, task_set: &mut TaskSet, cores: usize) -> SchedulabilityResult;
}