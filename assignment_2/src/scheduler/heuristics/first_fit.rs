use crate::models::TaskSet;
use crate::scheduler::errors::PartitionedError;
use crate::scheduler::heuristics::strategy::HeuristicStrategy;
use crate::scheduler::partitionned::Processor;

pub struct FirstFit;

impl HeuristicStrategy for FirstFit {
	fn assign_cores(&self, taskset: &mut TaskSet, cores: usize) -> Result<Vec<Processor>, PartitionedError> {
		// TODO: Parallelize this
		let mut processors: Vec<Processor> = vec![Processor::new(); cores];
		// For each task and for each processor
		// If the tasks fits in the processor, assign it
		for task in taskset.tasks() {
			let mut assigned = false;
			for processor in processors.iter_mut() {
				if processor.does_fit(task) {
					processor.add_task(task.clone());
					assigned = true;
					break;
				}
			}
			if !assigned {
				return Err(PartitionedError::CouldNotAssignTask { task: task.clone() });
			}
		}
		Ok(processors)
	}
}