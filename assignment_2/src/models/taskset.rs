use super::{Job, Task, TimeStep};
use num::Integer;

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

	pub fn utilization_max(&self) -> f64 {
		self.tasks.iter().map(|t| t.utilization()).max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap()
	}

	pub fn hyperperiod(&self) -> TimeStep {
		self.tasks.iter().fold(1, |acc, t| Self::checked_lcm(acc, t.period()).unwrap_or(TimeStep::MAX))
	}

	fn checked_lcm(a: TimeStep, b: TimeStep) -> Option<TimeStep> {
		a.checked_mul(b / a.gcd(&b))
	}

	pub fn len(&self) -> usize {
		self.tasks.len()
	}

	pub fn mut_tasks(&mut self) -> &mut Vec<Task> {
		&mut self.tasks
	}

	pub fn tasks(&mut self) -> &Vec<Task> {
		&self.tasks
	}

	pub fn is_implicit_deadline(&self) -> bool {
		self.tasks.iter().all(|t| t.deadline() == t.period())
	}

	pub fn set_highest_priority_on_task(&mut self, idx: usize, priority: bool) {
		self.tasks[idx].set_highest_priority(priority);
	}
}