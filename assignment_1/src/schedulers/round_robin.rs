use super::strategy::SchedulerStrategy;
use crate::{
	models::{Job, TaskSet, TimeStep},
	schedulers::errors::SchedulingError,
	schedulers::result::SchedulabilityResult,
	schedulers::simulator_strategy::SchedulerSimulatorStrategy,
};

pub struct RoundRobin;

impl SchedulerStrategy for RoundRobin {
	fn is_schedulable(&self, task_set: &mut TaskSet) -> SchedulabilityResult {
		if task_set.system_utilization() > 1.0 {
			return SchedulabilityResult::UnschedulableShortcut;
		}

		match self.simulate(task_set) {
			Ok(()) => SchedulabilityResult::SchedulableSimulated,
			Err(SchedulingError::DeadlineMiss { job: _job, t: _t }) => SchedulabilityResult::UnschedulableSimulated,
			// Err(_) => SchedulabilityResult::Unknown,
		}
	}
}

impl SchedulerSimulatorStrategy for RoundRobin {
	fn next_job<'a>(&'a self, queue: &'a mut Vec<Job>) -> Option<&'a mut Job> {
		if !queue.is_empty() {
			Some(&mut queue[0])
		} else {
			None
		}
	}

	fn t_max(&self, task_set: &TaskSet) -> TimeStep {
		task_set.hyperperiod()
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::models::{Job, Task, TaskSet};

	#[test]
	fn is_schedulable_returns_unschedulable_shortcut_when_utilization_exceeds_one() {
		let mut task_set = TaskSet::new(vec![
			Task::new(1, 0, 2, 2, 2),
			Task::new(2, 0, 2, 2, 2),
		]);
		let scheduler = RoundRobin;
		assert_eq!(scheduler.is_schedulable(&mut task_set), SchedulabilityResult::UnschedulableShortcut);
	}

	#[test]
	fn is_schedulable_returns_schedulable_simulated_when_simulation_succeeds() {
		let mut task_set = TaskSet::new(vec![
			Task::new(1, 0, 1, 3, 3),
		]);
		let scheduler = RoundRobin;
		assert_eq!(scheduler.is_schedulable(&mut task_set), SchedulabilityResult::SchedulableSimulated);
	}

	#[test]
	fn is_schedulable_returns_unschedulable_simulated_when_deadline_miss_occurs() {
		let mut task_set = TaskSet::new(vec![
			Task::new(1, 0, 2, 3, 3),
		]);
		let scheduler = RoundRobin;
		assert_eq!(scheduler.is_schedulable(&mut task_set), SchedulabilityResult::SchedulableSimulated);
	}

	#[test]
	fn is_schedulable_returns_unknown_when_other_error_occurs() {
		let mut task_set = TaskSet::new(vec![
			Task::new(1, 0, 3, 3, 3),
		]);
		let scheduler = RoundRobin;
		assert_eq!(scheduler.is_schedulable(&mut task_set), SchedulabilityResult::SchedulableSimulated);
	}

	#[test]
	fn next_job_returns_none_when_queue_is_empty() {
		let mut queue: Vec<Job> = Vec::new();
		let scheduler = RoundRobin;
		let job = scheduler.next_job(&mut queue);
		assert!(job.is_none());
	}

	#[test]
	fn t_max_calculates_lcm_of_all_periods() {
		let task_set = TaskSet::new(vec![
			Task::new(1, 0, 1, 3, 3),
			Task::new(2, 0, 1, 4, 4),
		]);
		let scheduler = RoundRobin;
		assert_eq!(scheduler.t_max(&task_set), 12);
	}
}