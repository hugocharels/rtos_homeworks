use crate::models::{TaskSet, TimeStep};
use crate::scheduler::{
	errors::SchedulingError,
	heuristics::strategy::HeuristicStrategy,
	orderings::strategy::OrderingStrategy,
	result::SchedulabilityResult,
	scheduler::Scheduler,
	simulator::SimpleMultiCoreSchedulerSimulator,
	simulator::SingleCorePartitionSchedulerSimulator,
};

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
	fn is_schedulable(&mut self, task_set: &mut TaskSet, cores: usize) -> SchedulabilityResult {
		if self.heuristic.is_none() || self.ordering.is_none() {
			return SchedulabilityResult::Unknown;
		} else if task_set.system_utilization() > cores as f64 || task_set.utilization_max() > 1.0 {
			return SchedulabilityResult::UnschedulableShortcut;
		} else if cores >= task_set.len() {
			return SchedulabilityResult::SchedulableShortcut;
		// TODO: Check if FF DU instead of false
		} else if false && task_set.system_utilization() <= (cores + 1) as f64 / 2f64 {
			return SchedulabilityResult::SchedulableShortcut;
		}

		// Simulate the scheduling
		match self.simulate(task_set, cores) {
			Ok(()) => SchedulabilityResult::SchedulableSimulated,
			Err(_) => SchedulabilityResult::UnschedulableSimulated,
		}
	}
}

impl SingleCorePartitionSchedulerSimulator for Partitioned {
	fn t_max(&self, task_set: &TaskSet) -> TimeStep {
		task_set.hyperperiod()
	}

	fn simulate(&self, task_set: &mut TaskSet) -> Result<(), SchedulingError> {
		todo!()
	}
}

impl SimpleMultiCoreSchedulerSimulator for Partitioned {
	fn simulate(&mut self, task_set: &mut TaskSet, cores: usize) -> Result<(), SchedulingError> {
		// Apply the ordering strategy
		self.ordering.as_ref().unwrap().apply_order(task_set);

		let splited_taskset: Vec<TaskSet> = self.heuristic.as_ref().unwrap().get_splited_taskset(task_set, cores).map_err(|e| SchedulingError::PartitionedError(e))?;

		// Simulate the scheduling for each partition in parallel
		// splited_taskset
		// 	.into_par_iter()
		// 	.try_for_each(|mut partition| {
		// 		<Self as SingleCorePartitionSchedulerSimulator>::simulate(self, &mut partition)
		// 	})
		Ok(())
	}
}