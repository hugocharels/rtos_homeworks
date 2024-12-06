use crate::models::{Job, TaskSet};
use crate::scheduler::heuristics::strategy::HeuristicStrategy;

pub struct NextFit;

impl HeuristicStrategy for NextFit {
	fn assign_cores(&self, taskset: &mut TaskSet, cores: usize) {
		todo!()
	}

	fn next_jobs<'a>(&'a self, queue: &'a mut Vec<Job>, cores: usize) -> Vec<&'a mut Job> {
		todo!()
	}
}
