use super::task::Task;

#[derive(Debug, Clone, PartialEq)]
pub struct Job {
	task: Task,
	deadline: u32,
	remaining_time: u32,
	id: u32,
}

impl Job {
	pub fn new(task: Task, deadline: u32, id: u32) -> Self {
		Self {
			remaining_time: task.computation_time(),
			task,
			deadline,
			id,
		}
	}

	pub fn deadline_missed(&self, t: u32) -> bool {
		self.remaining_time > 0 && t > self.deadline
	}

	pub fn is_complete(&self) -> bool {
		self.remaining_time == 0
	}

	pub fn schedule(&mut self, n_steps: u32) {
		self.remaining_time -= n_steps;
	}

	pub fn task(&self) -> &Task {
		&self.task
	}

	pub fn remaining_time(&self) -> u32 {
		self.remaining_time
	}
}
