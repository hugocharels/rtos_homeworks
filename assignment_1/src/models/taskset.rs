use super::{Job, Task, TimeStep};

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
			.iter_mut()
			.filter_map(|t| t.spawn_job(current_time))
			.collect()
	}

	pub fn system_utilization(&self) -> f64 {
		self.tasks.iter().map(|t| t.utilization()).sum()
	}

	pub fn len(&self) -> usize {
		self.tasks.len()
	}

	pub fn tasks(&self) -> &Vec<Task> {
		&self.tasks
	}

	pub fn is_implicit_deadline(&self) -> bool {
		self.tasks.iter().all(|t| t.deadline() == t.period())
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
