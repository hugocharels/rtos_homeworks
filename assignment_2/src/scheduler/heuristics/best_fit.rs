use crate::models::TaskSet;
use crate::scheduler::errors::PartitionedError;
use crate::scheduler::heuristics::strategy::{HeuristicStrategy, Processor};

pub struct BestFit;

impl HeuristicStrategy for BestFit {
	fn assign_cores(&self, task_set: &mut TaskSet, cores: usize) -> Result<Vec<Processor>, PartitionedError> {
		let mut processors: Vec<Processor> = vec![Processor::new(); cores];
		for task in task_set.tasks() {
			let mut assigned = false;
			processors.sort_by_key(|processor| std::cmp::Reverse(processor.utilization()));
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