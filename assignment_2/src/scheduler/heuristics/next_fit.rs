use crate::models::TaskSet;
use crate::scheduler::errors::PartitionedError;
use crate::scheduler::heuristics::strategy::HeuristicStrategy;
use crate::scheduler::partitionned::Processor;

pub struct NextFit;

impl HeuristicStrategy for NextFit {
	fn assign_cores(&self, taskset: &mut TaskSet, cores: usize) -> Result<Vec<Processor>, PartitionedError> {
		todo!()
	}
}
