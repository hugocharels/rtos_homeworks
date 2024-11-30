use crate::models::TaskSet;
use crate::scheduler::heuristics::strategy::HeuristicStrategy;
use crate::scheduler::orderings::strategy::OrderingStrategy;
use crate::scheduler::result::SchedulabilityResult;
use crate::scheduler::scheduler::Scheduler;

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
	fn is_schedulable(&self, taskset: &mut TaskSet, cores: &u32) -> SchedulabilityResult {
		if taskset.system_utilization() > *cores as f64 || taskset.utilization_max() > 1.0 {
			return SchedulabilityResult::UnschedulableShortcut;
		}


		SchedulabilityResult::Unknown
	}
}