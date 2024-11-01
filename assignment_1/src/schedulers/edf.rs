use super::strategy::SchedulerStrategy;
use crate::{
	models::TaskSet,
	schedulers::result::SchedulabilityResult,
};

pub struct EDF;

impl SchedulerStrategy for EDF {
	fn is_schedulable(&self, task_set: &mut TaskSet) -> SchedulabilityResult {
		if task_set.system_utilization() > 1.0 {
			return SchedulabilityResult::UnschedulableShortcut;
		}

		if task_set.is_implicit_deadline() {
			return SchedulabilityResult::SchedulableShortcut;
		}

		// Simulation

		SchedulabilityResult::Unknown

	}
}