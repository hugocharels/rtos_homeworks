use crate::scheduler::errors::SchedulingError;
use crate::scheduler::simulator::SimpleMultiCoreSchedulerSimulator;
use crate::{
	models::{Job, TaskSet},
	scheduler::orderings::decreasing::Decreasing,
	scheduler::orderings::strategy::OrderingStrategy,
	scheduler::result::SchedulabilityResult,
	scheduler::scheduler::Scheduler,
	scheduler::simulator::MultiCoreSchedulerSimulator,
};
use rayon::prelude::ParallelSliceMut;

pub struct EDF {
	k: usize,
}

impl EDF {
	pub fn new(k: usize) -> Self {
		Self { k }
	}

	fn set_k_highest_priorities(&self, task_set: &mut TaskSet) {
		// Order by utilization
		Decreasing.apply_order(task_set);
		for i in 0..self.k.min(task_set.len()) {
			task_set.set_highest_priority_on_task(i, true);
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
	fn is_schedulable(&mut self, task_set: &mut TaskSet, cores: usize) -> SchedulabilityResult {
		if task_set.system_utilization() > cores as f64 || task_set.utilization_max() > 1.0 {
			return SchedulabilityResult::UnschedulableShortcut;
		} else if cores >= task_set.len() {
			return SchedulabilityResult::SchedulableShortcut;
		}
		// Save the first k tasks so that they have the highest priority
		self.set_k_highest_priorities(task_set);

		if task_set.is_implicit_deadline() && cores >= (self.k - 1) + f64::ceil(task_set.tasks()[self.k + 1].utilization() / (1.0 - task_set.tasks()[self.k].utilization())) as usize {
			return SchedulabilityResult::SchedulableShortcut;
		}

		match MultiCoreSchedulerSimulator::simulate(self, task_set, cores) {
			Ok(()) => SchedulabilityResult::SchedulableSimulated,
			Err(_) => SchedulabilityResult::UnschedulableSimulated,
		}
	}
}

impl SimpleMultiCoreSchedulerSimulator for EDF {
	fn simulate(&self, task_set: &mut TaskSet, cores: usize) -> Result<(), SchedulingError> {
		<Self as MultiCoreSchedulerSimulator>::simulate(self, task_set, cores)
	}
}

impl MultiCoreSchedulerSimulator for EDF {
	fn next_jobs<'a>(&'a self, queue: &'a mut Vec<Job>, cores: usize) -> Vec<&'a mut Job> {
		// Sort the queue by deadline
		queue.par_sort_by_key(|job| self.get_priority(job));

		// Return the "cores" first jobs
		queue.iter_mut().take(cores).collect()
	}
}