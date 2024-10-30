use crate::{
	models::{Job, TaskSet},
	schedulers::errors::SchedulingError,
	schedulers::result::SchedulabilityResult,
};

pub trait SchedulerStrategy {
	fn is_schedulable(&self, task_set: &mut TaskSet) -> SchedulabilityResult;

	fn next_job<'a>(&'a self, queue: &'a mut Vec<Job>) -> Option<&'a mut Job> {
		todo!("Make another trait to avoid code duplication and this method");
	}

	fn t_max(&self, task_set: &TaskSet) -> u64 {
		todo!("Make another trait to avoid code duplication and this method");
	}

	fn simulate(&self, task_set: &mut TaskSet) -> Result<(), SchedulingError> {
		let mut queue = vec![];
		let t_max = self.t_max(task_set);
		for t in 0..t_max {
			// Release new jobs
			queue.extend(task_set.release_jobs(t));
			// Check for deadlines
			for job in &queue {
				if job.deadline_missed(t) {
					return Err(SchedulingError::DeadlineMiss {
						job: job.clone(),
						t,
					});
				}
			}
			if let Some(elected) = self.next_job(&mut queue) {
				elected.schedule(1);
			}
			// Only keep the jobs that are not complete. This is ne very efficient
			// since we should only check for `elected`, but it is to avoid fighting
			// the borrow checker.
			queue.retain(|j| !j.is_complete());
		}
		Ok(())
	}
}
