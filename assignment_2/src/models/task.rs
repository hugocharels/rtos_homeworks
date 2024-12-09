use super::{job::Job, TimeStep};

#[derive(Clone, Debug, PartialEq)]
pub struct Task {
	id: u32,
	offset: TimeStep,
	wcet: TimeStep,
	deadline: TimeStep,
	period: TimeStep,
	jobs_released: u32,
	highest_priority: bool,
}

impl Task {
	pub fn new(
		id: u32,
		offset: TimeStep,
		wcet: TimeStep,
		deadline: TimeStep,
		period: TimeStep,
	) -> Self {
		Self {
			id,
			offset,
			wcet,
			deadline,
			period,
			jobs_released: 0,
			highest_priority: false,
		}
	}

	pub fn spawn_job(&mut self, t: TimeStep) -> Option<Job> {
		// Not yet released
		if t < self.offset {
			return None;
		}
		// Not a time at which a job should be released
		if (t - self.offset) % self.period != 0 {
			return None;
		}
		self.jobs_released += 1;
		Some(Job::new(
			self.clone(),
			self.deadline + t,
			self.jobs_released,
		))
	}

	pub fn utilization(&self) -> f64 {
		self.wcet as f64 / self.period as f64
	}

	pub fn offset(&self) -> TimeStep {
		self.offset
	}

	pub fn wcet(&self) -> TimeStep {
		self.wcet
	}

	pub fn deadline(&self) -> TimeStep {
		self.deadline
	}

	pub fn period(&self) -> TimeStep {
		self.period
	}


	pub fn set_highest_priority(&mut self, new: bool) {
		self.highest_priority = new;
	}

	pub fn is_highest_priority(&self) -> bool {
		self.highest_priority
	}
}

#[cfg(test)]
mod tests {
	use super::Task;

	#[test]
	fn spawn_t0() {
		let mut t = Task::new(0, 0, 10, 20, 40);
		assert!(t.spawn_job(0).is_some());
		assert!(t.spawn_job(5).is_none());
	}

	#[test]
	fn spawn_offset() {
		let mut t = Task::new(0, 5, 10, 20, 40);
		assert!(t.spawn_job(0).is_none());
		assert!(t.spawn_job(5).is_some());
	}
}
