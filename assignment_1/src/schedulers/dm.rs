use super::strategy::SchedulerStrategy;
use crate::{
	models::TaskSet,
	schedulers::result::SchedulabilityResult,
};
use rayon::prelude::*;

pub struct DM;

impl DM {
	fn check_deadline_constraints(&self, task_set: &TaskSet) -> bool {
		(0..task_set.len())
			.into_par_iter()
			.all(|i| {
				let task = &task_set.tasks()[i];
				let mut w = task.wcet() as f64;
				loop {
					let mut w_next = task.wcet() as f64;
					for j in 0..i {
						w_next += (w / task_set.tasks()[j].period() as f64).ceil()
							* (task_set.tasks()[j].wcet() as f64);
					}
					if w == w_next {
						break;
					}
					w = w_next;
				}
				w <= task.deadline() as f64
			})
	}
}

impl SchedulerStrategy for DM {
	fn is_schedulable(&self, task_set: &mut TaskSet) -> SchedulabilityResult {
		if task_set.system_utilization() > 1.0 {
			return SchedulabilityResult::UnschedulableShortcut;
		}

		task_set.sort_by_deadline();

		if self.check_deadline_constraints(task_set) {
			SchedulabilityResult::SchedulableShortcut
		} else {
			SchedulabilityResult::UnschedulableShortcut
		}
	}
}
