use super::{Job, Task, TimeStep};
use num::Integer;
use rayon::prelude::*;

#[derive(Clone, Debug)]
pub struct TaskSet {
	tasks: Vec<Task>,
}

impl TaskSet {
	pub fn new(tasks: Vec<Task>) -> Self {
		Self { tasks }
	}

	pub fn release_jobs(&mut self, current_time: TimeStep) -> Vec<Job> {
		self.tasks
			.par_iter_mut()
			.filter_map(|t| t.spawn_job(current_time))
			.collect()
	}

	pub fn sort_by_deadline(&mut self) {
		self.tasks.sort_by_key(|task| task.deadline());
	}

	pub fn system_utilization(&self) -> f64 {
		self.tasks.par_iter().map(|t| t.utilization()).sum()
	}

	pub fn utilization_max(&self) -> f64 {
		self.tasks.par_iter().map(|t| t.utilization()).max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap()
	}

	pub fn len(&self) -> usize {
		self.tasks.len()
	}

	pub fn tasks(&self) -> &Vec<Task> {
		&self.tasks
	}

	pub fn is_implicit_deadline(&self) -> bool {
		self.tasks.par_iter().all(|t| t.deadline() == t.period())
	}

	pub fn set_highest_priority_on_task(&mut self, idx: usize, priority: bool) {
		self.tasks[idx].set_highest_priority(priority);
	}
}

#[cfg(test)]
mod tests {
	use crate::models::Task;

	use super::TaskSet;

	#[test]
	fn test_taskset() {
		let tasks = vec![Task::new(0, 0, 10, 20, 50), Task::new(1, 0, 20, 40, 40)];
		let mut ts = TaskSet::new(tasks);
		assert_eq!(ts.release_jobs(0).len(), 2);
		assert!(ts.release_jobs(3).is_empty());
	}
}
