use crate::{
	models::{Job, TaskSet, TimeStep},
	schedulers::errors::SchedulingError,
};
use rayon::prelude::*;


pub trait SchedulerSimulatorStrategy {
	fn next_job<'a>(&'a self, queue: &'a mut Vec<Job>) -> Option<&'a mut Job>;

	fn t_max(&self, task_set: &TaskSet) -> TimeStep;

	fn simulate(&self, task_set: &mut TaskSet) -> Result<(), SchedulingError> {
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

			// Schedule the next job if available
			if let Some(elected) = self.next_job(&mut queue) {
				elected.schedule(1);
			}

			// Filter out completed jobs in parallel and create a new queue
			queue = queue
				.into_par_iter()
				.filter(|j| !j.is_complete())
				.collect();
		}
		Ok(())
	}
}
