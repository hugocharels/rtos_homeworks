use crate::{
	models::{Job, TaskSet},
	scheduler::orderings::decreasing::Decreasing,
	scheduler::orderings::strategy::OrderingStrategy,
	scheduler::result::SchedulabilityResult,
	scheduler::scheduler::Scheduler,
	scheduler::simulator::SchedulerSimulator,
};
use rayon::prelude::ParallelSliceMut;

pub struct EDF {
	k: usize,
}

impl EDF {
	pub fn new(k: usize) -> Self {
		Self { k }
	}

	fn set_k_highest_priorities(&self, taskset: &mut TaskSet) {
		// Order by utilization
		Decreasing.apply_order(taskset);
		for i in 0..self.k.min(taskset.len()) {
			taskset.set_highest_priority_on_task(i, true);
		}
	}

	fn get_priority(&self, job: &Job) -> u32 {
		if job.task().is_highest_priority() {
			0
		} else {
			job.deadline()
		}
	}
}

impl Scheduler for EDF {
	fn is_schedulable(&mut self, taskset: &mut TaskSet, cores: usize) -> SchedulabilityResult {
		if taskset.system_utilization() > cores as f64 || taskset.utilization_max() > 1.0 {
			return SchedulabilityResult::UnschedulableShortcut;
		} else if cores >= taskset.len() {
			return SchedulabilityResult::SchedulableShortcut;
		}
		// Save the first k tasks so that they have the highest priority
		self.set_k_highest_priorities(taskset);

		if taskset.is_implicit_deadline() && cores >= (self.k - 1) + f64::ceil(taskset.tasks()[self.k + 1].utilization() / (1.0 - taskset.tasks()[self.k].utilization())) as usize {
			return SchedulabilityResult::SchedulableShortcut;
		}

		match self.simulate(taskset, cores) {
			Ok(()) => SchedulabilityResult::SchedulableSimulated,
			Err(_) => SchedulabilityResult::UnschedulableSimulated,
		}
	}
}

impl SchedulerSimulator for EDF {
	fn next_jobs<'a>(&'a mut self, queue: &'a mut Vec<Job>, cores: usize) -> Vec<&'a mut Job> {
		// Sort the queue by deadline
		queue.par_sort_by_key(|job| self.get_priority(job));

		// Return the "cores" first jobs
		queue.iter_mut().take(cores).collect()
	}
}