use super::strategy::SchedulerStrategy;
use crate::{
	models::TaskSet,
	schedulers::result::SchedulabilityResult,
};

pub struct EDF;

impl SchedulerStrategy for EDF {
	fn is_schedulable(&self, task_set: &mut TaskSet) -> SchedulabilityResult {
		match task_set.system_utilization() <= 1.0 {
			true => SchedulabilityResult::SchedulableShortcut,
			false => SchedulabilityResult::UnschedulableShortcut,
		}
	}
}