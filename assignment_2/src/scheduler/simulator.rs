use crate::{
	models::{Job, TaskSet, TimeStep},
	scheduler::errors::SchedulingError,
};


pub trait SingleCorePartitionSchedulerSimulator {
	fn t_max(&self, task_set: &TaskSet) -> TimeStep;
	fn simulate(&self, task_set: &mut TaskSet) -> Result<(), SchedulingError>;
}


pub trait SimpleMultiCoreSchedulerSimulator {
	fn simulate(&self, task_set: &mut TaskSet, cores: usize) -> Result<(), SchedulingError>;
}


pub trait MultiCoreSchedulerSimulator: SimpleMultiCoreSchedulerSimulator {
	fn next_jobs<'a>(&'a self, queue: &'a mut Vec<Job>, cores: usize) -> Vec<&'a mut Job>;

	fn t_max(&self, task_set: &TaskSet) -> TimeStep {
		// TODO: [O_max, O_max + 2P)
		1_000_000.min(task_set.hyperperiod())
	}

	fn simulate(&self, task_set: &mut TaskSet, cores: usize) -> Result<(), SchedulingError> {
		let mut queue = vec![];
		let t_max = self.t_max(task_set);

		for t in 0..t_max {

			// Release new jobs in parallel
			queue.extend(task_set.release_jobs(t));

			// Check for deadlines in parallel
			if let Some(missed_job) = queue.iter().find(|job| job.deadline_missed(t)) {
				return Err(SchedulingError::DeadlineMiss {
					job: missed_job.clone(),
					t,
				});
			}

			// Get up to `cores` jobs to schedule
			let mut scheduled_jobs = self.next_jobs(&mut queue, cores);

			// Simulate the execution of scheduled jobs in parallel
			scheduled_jobs.iter_mut().for_each(|job| job.schedule(1));

			// Filter out completed jobs in parallel and create a new queue
			queue = queue
				.into_iter()
				.filter(|j| !j.is_complete())
				.collect();
		}
		Ok(())
	}
}
