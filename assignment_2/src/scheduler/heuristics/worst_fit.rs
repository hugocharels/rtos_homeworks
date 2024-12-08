use crate::models::TaskSet;
use crate::scheduler::errors::PartitionedError;
use crate::scheduler::heuristics::strategy::HeuristicStrategy;
use crate::scheduler::partitioned::Processor;

pub struct WorstFit;

impl HeuristicStrategy for WorstFit {
	fn assign_cores(&self, taskset: &mut TaskSet, cores: usize) -> Result<Vec<Processor>, PartitionedError> {
		// TODO: Parallelize this
		let mut processors: Vec<Processor> = vec![Processor::new(); cores];
		for task in taskset.tasks() {
			let mut assigned = false;
			processors.sort_by_key(|processor| processor.utilization());
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