use super::strategy::SchedulerStrategy;
use crate::{
	models::TaskSet,
	schedulers::result::SchedulabilityResult,
};

pub struct SchedulerContext<'a> {
	strategy: Option<&'a dyn SchedulerStrategy>,  // Strategy can be set later
	task_set: TaskSet,  // &self, TaskSet is an attribute of the context
}

impl<'a> SchedulerContext<'a> {
	pub fn new(task_set: TaskSet) -> Self {
		Self {
			strategy: None,  // Initially, no strategy is set
			task_set,
		}
	}

	pub fn set_strategy(&mut self, strategy: &'a dyn SchedulerStrategy) {
		self.strategy = Some(strategy);
	}

	pub fn check_schedulability(&self) -> SchedulabilityResult {
		match self.strategy {
			Some(strategy) => strategy.is_schedulable(&mut self.task_set.clone()),
			None => panic!("No strategy set!"),
		}
	}
}
