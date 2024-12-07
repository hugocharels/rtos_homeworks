use crate::models::TaskSet;
use crate::scheduler::errors::PartitionedError;
use crate::scheduler::heuristics::strategy::HeuristicStrategy;
use crate::scheduler::partitionned::Processor;

pub struct BestFit;

impl HeuristicStrategy for BestFit {
	fn assign_cores(&self, taskset: &mut TaskSet, cores: usize) -> Result<Vec<Processor>, PartitionedError> {
		todo!()
	}
}