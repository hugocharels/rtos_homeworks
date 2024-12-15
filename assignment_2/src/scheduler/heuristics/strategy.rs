use crate::{
	models::{Task, TaskSet, TimeStep},
	scheduler::errors::PartitionedError,
};

pub trait HeuristicStrategy: Sync + Send {
	fn assign_cores(&self, task_set: &mut TaskSet, cores: usize) -> Result<Vec<Processor>, PartitionedError>;

	fn get_splited_taskset(&self, task_set: &mut TaskSet, cores: usize) -> Result<Vec<TaskSet>, PartitionedError> {
		// Call assign cores then create a vec of taskset where each taskset is the tasks of a processor
		let processors = self.assign_cores(task_set, cores)?;
		let mut tasksets = Vec::new();
		for processor in processors {
			let new_taskset = TaskSet::new(processor.tasks().clone());
			tasksets.push(new_taskset);
		}
		Ok(tasksets)
	}

	fn is_ff(&self) -> bool {
		false
	}
}

#[derive(Clone)]
pub struct Processor {
	utilization: TimeStep,
	tasks: Vec<Task>,
}

impl Processor {
	pub fn new() -> Processor {
		Self {
			utilization: 0,
			tasks: Vec::new(),
		}
	}

	pub fn does_fit(&self, task: &Task) -> bool {
		self.utilization + task.wcet() <= task.deadline()
	}

	pub fn add_task(&mut self, task: Task) {
		if self.tasks.is_empty() {
			self.utilization = task.offset();
		}
		self.utilization += task.wcet();
		self.tasks.push(task);
	}

	pub fn utilization(&self) -> TimeStep {
		self.utilization
	}

	pub fn tasks(&self) -> &Vec<Task> {
		&self.tasks
	}
}
