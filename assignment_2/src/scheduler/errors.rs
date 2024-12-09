use crate::models::{Job, Task, TimeStep};

#[derive(Debug)]
pub enum SchedulingError {
	DeadlineMiss { job: Job, t: TimeStep },
	PartitionedError(PartitionedError),
}

#[derive(Debug)]
pub enum PartitionedError {
	CouldNotAssignTask { task: Task },
}