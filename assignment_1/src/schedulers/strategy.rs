use crate::{
	models::TaskSet,
	schedulers::result::SchedulabilityResult,
};

pub trait SchedulerStrategy {
	fn is_schedulable(&self, task_set: &mut TaskSet) -> SchedulabilityResult;
}
