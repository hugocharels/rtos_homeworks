use super::Task;
use crate::models::TimeStep;

#[derive(Debug, Clone, PartialEq)]
pub struct Job {
	task: Task,
	deadline: TimeStep,
	remaining_time: TimeStep,
	id: u32,
}

impl Job {
	pub fn new(task: Task, deadline: TimeStep, id: u32) -> Self {
		Self {
			remaining_time: task.wcet(),
			task,
			deadline,
			id,
		}
	}

	pub fn deadline_missed(&self, t: TimeStep) -> bool {
		self.remaining_time > 0 && t > self.deadline
	}

	pub fn is_complete(&self) -> bool {
		self.remaining_time == 0
	}

	pub fn schedule(&mut self, n_steps: TimeStep) {
		self.remaining_time -= n_steps;
	}

	pub fn deadline(&self) -> TimeStep {
		self.deadline
	}

	pub fn task(&self) -> &Task {
		&self.task
	}
}