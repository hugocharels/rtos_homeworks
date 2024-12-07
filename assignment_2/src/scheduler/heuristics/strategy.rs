use crate::models::TaskSet;
use crate::scheduler::errors::PartitionedError;
use crate::scheduler::partitionned::Processor;

pub trait HeuristicStrategy {
	fn assign_cores(&self, taskset: &mut TaskSet, cores: usize) -> Result<Vec<Processor>, PartitionedError>;
}