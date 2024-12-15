use crate::{
	models::TaskSet,
	scheduler::{
		errors::PartitionedError,
		heuristics::strategy::{HeuristicStrategy, Processor},
	},
};

pub struct NextFit;

impl HeuristicStrategy for NextFit {
	fn assign_cores(&self, task_set: &mut TaskSet, cores: usize) -> Result<Vec<Processor>, PartitionedError> {
		let mut processors: Vec<Processor> = vec![Processor::new(); cores];
		let mut current_processor = 0;
		for task in task_set.tasks() {
			let mut assigned = false;
			for i in current_processor..processors.len() {
				let processor = &mut processors[i];
				if processor.does_fit(task) {
					processor.add_task(task.clone());
					assigned = true;
					current_processor = i;
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
