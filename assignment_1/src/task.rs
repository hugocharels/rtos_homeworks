use super::job::Job;

#[derive(Clone, Debug, PartialEq)]
pub struct Task {
	id: u32,
	offset: u32,
	computation_time: u32,
	deadline: u32,
	period: u32,
	jobs_released: u32,
}

impl Task {
	pub fn new(
		id: u32,
		offset: u32,
		computation_time: u32,
		deadline: u32,
		period: u32,
	) -> Self {
		Self {
			id,
			offset,
			computation_time,
			deadline,
			period,
			jobs_released: 0,
		}
	}

	pub fn spawn_job(&mut self, t: u32) -> Option<Job> {
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

	pub fn period(&self) -> u32 {
		self.period
	}

	pub fn id(&self) -> u32 {
		self.id
	}

	pub fn computation_time(&self) -> u32 {
		self.computation_time
	}
}
