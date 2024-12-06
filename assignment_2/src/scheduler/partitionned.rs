use crate::models::{Job, TaskSet};
use crate::scheduler::{
	heuristics::strategy::HeuristicStrategy,
	orderings::strategy::OrderingStrategy,
	result::SchedulabilityResult,
	scheduler::Scheduler,
};
use crate::scheduler::simulator::SchedulerSimulator;

pub struct Partitioned {
	heuristic: Option<Box<dyn HeuristicStrategy>>,
	ordering: Option<Box<dyn OrderingStrategy>>,
}

impl Partitioned {
	pub fn new() -> Partitioned {
		Self {
			heuristic: None,
			ordering: None,
		}
	}

	pub fn set_heuristic(&mut self, heuristic: Box<dyn HeuristicStrategy>) {
		self.heuristic = Some(heuristic);
	}

	pub fn set_ordering(&mut self, ordering: Box<dyn OrderingStrategy>) {
		self.ordering = Some(ordering);
	}
}

impl Scheduler for Partitioned {
	fn is_schedulable(&self, taskset: &mut TaskSet, cores: usize) -> SchedulabilityResult {
		if self.heuristic.is_none() || self.ordering.is_none() {
			return SchedulabilityResult::Unknown;
		} else if taskset.system_utilization() > cores as f64 || taskset.utilization_max() > 1.0 {
			return SchedulabilityResult::UnschedulableShortcut;
		} else if cores >= taskset.len() {
			return SchedulabilityResult::SchedulableShortcut;
		// TODO: Check if FF DU instead of false
		} else if false && taskset.system_utilization() <= (cores + 1) as f64 / 2f64 {
			return SchedulabilityResult::SchedulableShortcut;
		}

		// Apply the ordering strategy
		self.ordering.as_ref().unwrap().apply_order(taskset);
		// Apply the heuristic strategy
		self.heuristic.as_ref().unwrap().assign_cores(taskset, cores);

		match self.simulate(taskset, cores) {
			Ok(()) => SchedulabilityResult::SchedulableSimulated,
			Err(_) => SchedulabilityResult::UnschedulableSimulated,
		}
	}
}

impl SchedulerSimulator for Partitioned {
	fn next_jobs<'a>(&'a self, queue: &'a mut Vec<Job>, cores: usize) -> Vec<&'a mut Job> {
		self.heuristic.as_ref().unwrap().next_jobs(queue, cores)
	}
}