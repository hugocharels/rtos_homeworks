use crate::{
	models::{Job, TaskSet, TimeStep},
	scheduler::errors::SchedulingError,
};
use rayon::prelude::*;


pub trait SchedulerSimulator {
	fn next_jobs<'a>(&'a self, queue: &'a mut Vec<Job>, cores: &u32) -> Vec<&'a mut Job>;

	fn t_max(&self, task_set: &TaskSet) -> TimeStep;

	fn simulate(&self, task_set: &mut TaskSet, cores: &u32) -> Result<(), SchedulingError> {
		let mut queue = vec![];
		let t_max = self.t_max(task_set);

		for t in 0..t_max {
			// Release new jobs in parallel
			queue.par_extend(task_set.release_jobs(t));

			// Check for deadlines in parallel
			if queue.par_iter().any(|job| job.deadline_missed(t)) {
				return Err(SchedulingError::DeadlineMiss {
					// Find the first job that missed the deadline (for detailed error reporting)
					job: queue.iter().find(|job| job.deadline_missed(t)).unwrap().clone(),
					t,
				});
			}

			// Get up to `cores` jobs to schedule
			let scheduled_jobs = self.next_jobs(&mut queue, cores);

			// Simulate the execution of scheduled jobs in parallel
			scheduled_jobs.par_iter().for_each(|mut job| job.schedule(1));

			// Filter out completed jobs in parallel and create a new queue
			queue = queue
				.into_par_iter()
				.filter(|j| !j.is_complete())
				.collect();
		}
		Ok(())
	}
}
